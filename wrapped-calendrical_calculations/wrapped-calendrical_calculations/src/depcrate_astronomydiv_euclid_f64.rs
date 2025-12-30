// Generated macro for div_euclid_f64 (function)
macro_rules! Depcrate_astronomydiv_euclid_f64 {
() => {
// Module: crate::astronomy
// Provides: {"div_euclid_f64"}
// Dependencies: {}
fn div_euclid_f64 (n : f64 , d : f64) -> f64 { debug_assert ! (d > 0.0) ; let (a , b) = (n / d , n % d) ; if n >= 0.0 || b == 0.0 { a } else { a - 1.0 } }
};
}
