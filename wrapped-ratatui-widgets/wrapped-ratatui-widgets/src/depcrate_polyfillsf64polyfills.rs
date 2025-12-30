// Generated macro for F64Polyfills (trait)
macro_rules! Depcrate_polyfillsF64Polyfills {
() => {
// Module: crate::polyfills
// Provides: {"F64Polyfills"}
// Dependencies: {}
pub (crate) trait F64Polyfills { # [doc = " Computes `(self * a) + b`."] fn mul_add (self , a : f64 , b : f64) -> f64 ; # [doc = " Returns the nearest integer to `self`. If a value is half-way between two integers, round"] # [doc = " away from `0.0`."] fn round (self) -> f64 ; # [doc = " Returns the largest integer less than or equal to `self`."] fn floor (self) -> f64 ; # [doc = " Approximates the sine of a number (in radians) with max error of `0.002`."] fn sin (self) -> f64 ; # [doc = " Approximates the cosine of a number (in radians) with max error of `0.002`."] fn cos (self) -> f64 ; }
};
}
