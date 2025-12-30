// Generated macro for erfc1 (function)
macro_rules! Depcrate_math_erferfc1 {
() => {
// Module: crate::math::erf
// Provides: {"erfc1"}
// Dependencies: {}
fn erfc1 (x : f64) -> f64 { let s : f64 ; let p : f64 ; let q : f64 ; s = fabs (x) - 1.0 ; p = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6))))) ; q = 1.0 + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6))))) ; 1.0 - ERX - p / q }
};
}
