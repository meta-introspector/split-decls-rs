// Generated macro for sin_pi (function)
macro_rules! Depcrate_math_lgamma_rsin_pi {
() => {
// Module: crate::math::lgamma_r
// Provides: {"sin_pi"}
// Dependencies: {}
fn sin_pi (mut x : f64) -> f64 { let mut n : i32 ; x = 2.0 * (x * 0.5 - floor (x * 0.5)) ; n = (x * 4.0) as i32 ; n = div ! (n + 1 , 2) ; x -= (n as f64) * 0.5 ; x *= PI ; match n { 1 => k_cos (x , 0.0) , 2 => k_sin (- x , 0.0 , 0) , 3 => - k_cos (x , 0.0) , _ => k_sin (x , 0.0 , 0) , } }
};
}
