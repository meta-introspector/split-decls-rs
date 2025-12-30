// Generated macro for poly (function)
macro_rules! Depcrate_helperspoly {
() => {
// Module: crate::helpers
// Provides: {"poly"}
// Dependencies: {}
pub (crate) fn poly (x : f64 , coeffs : & [f64]) -> f64 { coeffs . iter () . rev () . fold (0.0 , | a , c | a * x + c) }
};
}
