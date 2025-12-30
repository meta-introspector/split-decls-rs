// Generated macro for sinpi (function)
macro_rules! Depcrate_math_tgammasinpi {
() => {
// Module: crate::math::tgamma
// Provides: {"sinpi"}
// Dependencies: {}
fn sinpi (mut x : f64) -> f64 { let mut n : isize ; x = x * 0.5 ; x = 2.0 * (x - floor (x)) ; n = (4.0 * x) as isize ; n = div ! (n + 1 , 2) ; x -= (n as f64) * 0.5 ; x *= PI ; match n { 1 => k_cos (x , 0.0) , 2 => k_sin (- x , 0.0 , 0) , 3 => - k_cos (x , 0.0) , _ => k_sin (x , 0.0 , 0) , } }
};
}
