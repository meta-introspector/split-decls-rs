// Generated macro for box_kernel (function)
macro_rules! Depcrate_imageops_samplebox_kernel {
() => {
// Module: crate::imageops::sample
// Provides: {"box_kernel"}
// Dependencies: {}
# [doc = " Calculate the box kernel."] # [doc = " Only pixels inside the box should be considered, and those"] # [doc = " contribute equally.  So this method simply returns 1."] pub (crate) fn box_kernel (_x : f32) -> f32 { 1.0 }
};
}
