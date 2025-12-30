// Generated macro for pad_shape_scalar (function)
macro_rules! Depcrate_core_geometrypad_shape_scalar {
() => {
// Module: crate::core::geometry
// Provides: {"pad_shape_scalar"}
// Dependencies: {}
# [doc = " Increase the size of X and Y by \\p s."] pub fn pad_shape_scalar (size : Point , s : f64) -> Point { Point :: new (size . x + s , size . y + s) }
};
}
