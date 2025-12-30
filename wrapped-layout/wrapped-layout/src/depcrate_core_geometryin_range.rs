// Generated macro for in_range (function)
macro_rules! Depcrate_core_geometryin_range {
() => {
// Module: crate::core::geometry
// Provides: {"in_range"}
// Dependencies: {}
# [doc = " \\return true if \\p x is in the inclusive range P.x .. P.y."] pub fn in_range (range : (f64 , f64) , x : f64) -> bool { x >= range . 0 && x <= range . 1 }
};
}
