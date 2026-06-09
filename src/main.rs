use raylib::prelude::*;

const GAME_WIDTH: f32 = 800.0;
const GAME_HEIGHT: f32 = 600.0;

struct Ball {
    prect: Rectangle, // posRect: use x and y in Rectangle struct to present ball position
    velocity: Vector2,
    radius: f32,
    color: Color,
    is_active: bool,
}

struct Paddle {
    rect: Rectangle,
    color: Color,
    is_active: bool,
}

fn paddle_check_colision_up(paddle: &Paddle) -> bool {
    paddle.rect.y >= 0.0
}

fn paddle_check_colision_down(paddle: &Paddle) -> bool {
    paddle.rect.y <= GAME_HEIGHT - paddle.rect.height
}

fn ball_check_collision_x(ball: &Ball) -> bool {
    ball.prect.x <= 0.0 || ball.prect.x >= GAME_WIDTH - ball.radius * 2.0
}

fn ball_check_collision_y(ball: &Ball) -> bool {
    ball.prect.y <= 0.0 || ball.prect.y >= GAME_HEIGHT - ball.radius * 2.0
}

fn aabb_collision_algorithm(pad: &Paddle, ball: &Ball) -> bool {
    pad.rect.x < ball.prect.x + ball.radius * 2.0
        && pad.rect.x + pad.rect.width > ball.prect.x
        && pad.rect.y < ball.prect.y + ball.radius * 2.0
        && pad.rect.y + pad.rect.height > ball.prect.y
}

fn main() {
    let (mut rl, thread) = raylib::init()
        // .fullscreen()
        // .resizable()
        .width(GAME_WIDTH as i32)
        .height(GAME_HEIGHT as i32)
        .title("Hello Raylib")
        .build();

    let ball_radius = 10.0;
    let mut ball = Ball {
        prect: Rectangle::new(
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0,
            ball_radius * 2.0,
            ball_radius * 2.0,
        ),
        velocity: Vector2::new(350.0, -350.0),
        radius: ball_radius,
        color: Color::RED,
        is_active: true,
    };

    let paddle_w = GAME_HEIGHT / 5.0;
    let paddle_h = 20.0;

    let mut l_paddle = Paddle {
        rect: Rectangle::new(
            GAME_WIDTH / 10.0,
            (GAME_HEIGHT - paddle_w) / 2.0,
            paddle_h,
            paddle_w,
        ),
        color: Color::RED,
        is_active: true,
    };

    let mut r_paddle = Paddle {
        rect: Rectangle::new(
            GAME_WIDTH - (GAME_WIDTH / 10.0),
            (GAME_HEIGHT - paddle_w) / 2.0,
            paddle_h,
            paddle_w,
        ),
        color: Color::BLUE,
        is_active: true,
    };

    let mut l_score = 0;
    let mut r_score = 0;

    rl.set_target_fps(60);
    let mut is_running = true;
    while !rl.window_should_close() && is_running {
        let dt = rl.get_frame_time();

        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            is_running = false;
        }

        if r_paddle.is_active || l_paddle.is_active {
            if rl.is_key_down(KeyboardKey::KEY_W) && paddle_check_colision_up(&l_paddle) {
                l_paddle.rect.y -= 10.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) && paddle_check_colision_down(&l_paddle) {
                l_paddle.rect.y += 10.0;
            }

            if rl.is_key_down(KeyboardKey::KEY_LEFT) && paddle_check_colision_up(&r_paddle) {
                r_paddle.rect.y -= 10.0;
            }

            if rl.is_key_down(KeyboardKey::KEY_RIGHT) && paddle_check_colision_down(&r_paddle) {
                r_paddle.rect.y += 10.0;
            }
        }

        if ball.is_active {
            ball.prect.x += ball.velocity.x * dt;
            ball.prect.y += ball.velocity.y * dt;

            if ball_check_collision_x(&ball) {
                // is_running = false;
                if ball.prect.x <= 0.0 {
                    r_score += 1;
                } else {
                    l_score += 1;
                }

                ball.prect.x = GAME_WIDTH / 2.0;
                ball.prect.y = GAME_HEIGHT / 2.0;

                ball.velocity *= -1.0;
            }

            if ball_check_collision_y(&ball) {
                ball.velocity.y *= -1.0;
            }

            if aabb_collision_algorithm(&r_paddle, &ball)
                || aabb_collision_algorithm(&l_paddle, &ball)
            {
                ball.velocity.x *= -1.0;

                if ball.velocity.x > 0.0 {
                    ball.prect.x = l_paddle.rect.x + l_paddle.rect.width + 1.0;
                } else {
                    ball.prect.x = r_paddle.rect.x - (ball.radius * 2.0) - 1.0;
                }
            }
        }

        let font_size = 20;
        let print_dt = format!("delta time = {}", dt);

        let print_score = format!("Score {} : {}", l_score, r_score);
        let text_width = rl.measure_text(&print_score, 20);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_circle(
            ball.prect.x as i32,
            ball.prect.y as i32,
            ball.radius,
            ball.color,
        );

        d.draw_rectangle_rec(r_paddle.rect, r_paddle.color);
        d.draw_rectangle_rec(l_paddle.rect, l_paddle.color);

        d.draw_text(&print_dt, 0, 0, font_size, Color::BLACK);
        d.draw_text(
            &print_score,
            (GAME_WIDTH as i32 - text_width) / 2,
            GAME_HEIGHT as i32 - font_size,
            font_size,
            Color::BLACK,
        );
    }
}
