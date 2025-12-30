// Generated macro for invert_angular (function)
macro_rules! Depcrate_helpersinvert_angular {
() => {
// Module: crate::helpers
// Provides: {"invert_angular"}
// Dependencies: {}
pub (crate) fn invert_angular < F : Fn (f64) -> f64 > (f : F , y : f64 , r : (f64 , f64)) -> f64 { binary_search (r . 0 , r . 1 , | x | (f (x) - y) . rem_euclid (360.0) < 180.0 , 1e-5) }
};
}
