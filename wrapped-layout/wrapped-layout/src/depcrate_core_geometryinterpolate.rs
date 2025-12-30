// Generated macro for interpolate (function)
macro_rules! Depcrate_core_geometryinterpolate {
() => {
// Module: crate::core::geometry
// Provides: {"interpolate"}
// Dependencies: {}
# [doc = " Perform linear interpolation of the vectors v0 and v1, using the"] # [doc = " ratio w which is assumed to be between 0..1."] pub fn interpolate (v0 : Point , v1 : Point , w : f64) -> Point { v0 . scale (w) . add (v1 . scale (1. - w)) }
};
}
