// Generated macro for floor (function)
macro_rules! Depcrate_polyfillsfloor {
() => {
// Module: crate::polyfills
// Provides: {"floor"}
// Dependencies: {}
# [inline] fn floor (val : f64) -> f64 { let mut res = (val as i64) as f64 ; if val < res { res -= 1.0 ; } res }
};
}
