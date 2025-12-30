// Generated macro for mandelbrot_at_point (function)
macro_rules! Depcrate_mandelbrotmandelbrot_at_point {
() => {
// Module: crate::mandelbrot
// Provides: {"mandelbrot_at_point"}
// Dependencies: {}
fn mandelbrot_at_point (cx : f64 , cy : f64) -> u32 { let mut x = 0.0 ; let mut y = 0.0 ; let mut iter = 0 ; while iter < MAX_ITER && x * x + y * y <= 4.0 { let xtemp = x * x - y * y + cx ; y = 2.0 * x * y + cy ; x = xtemp ; iter += 1 ; } iter }
};
}
