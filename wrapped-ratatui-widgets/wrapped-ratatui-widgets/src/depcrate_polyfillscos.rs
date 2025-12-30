// Generated macro for cos (function)
macro_rules! Depcrate_polyfillscos {
() => {
// Module: crate::polyfills
// Provides: {"cos"}
// Dependencies: {}
# [inline] fn cos (val : f64) -> f64 { let mut x = val ; x *= FRAC_1_PI / 2.0 ; x -= 0.25 + floor (x + 0.25) ; x *= 16.0 * (x . abs () - 0.5) ; x += 0.225 * x * (x . abs () - 1.0) ; x }
};
}
