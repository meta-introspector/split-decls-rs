// Generated macro for round (function)
macro_rules! Depcrate_polyfillsround {
() => {
// Module: crate::polyfills
// Provides: {"round"}
// Dependencies: {}
# [inline] fn round (val : f64) -> f64 { (val + 0.5f64 . copysign (val)) as i64 as f64 }
};
}
