// Generated macro for catmullrom_kernel (function)
macro_rules! Depcrate_imageops_samplecatmullrom_kernel {
() => {
// Module: crate::imageops::sample
// Provides: {"catmullrom_kernel"}
// Dependencies: {}
# [doc = " Calculate the Catmull-Rom cubic spline."] # [doc = " Also known as a form of `BiCubic` sampling in two dimensions."] pub (crate) fn catmullrom_kernel (x : f32) -> f32 { bc_cubic_spline (x , 0.0 , 0.5) }
};
}
