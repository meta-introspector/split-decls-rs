// Generated macro for triangle_kernel (function)
macro_rules! Depcrate_imageops_sampletriangle_kernel {
() => {
// Module: crate::imageops::sample
// Provides: {"triangle_kernel"}
// Dependencies: {}
# [doc = " Calculate the triangle function."] # [doc = " Also known as `BiLinear` sampling in two dimensions."] pub (crate) fn triangle_kernel (x : f32) -> f32 { if x . abs () < 1.0 { 1.0 - x . abs () } else { 0.0 } }
};
}
