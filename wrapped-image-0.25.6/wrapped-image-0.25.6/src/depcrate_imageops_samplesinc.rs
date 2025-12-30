// Generated macro for sinc (function)
macro_rules! Depcrate_imageops_samplesinc {
() => {
// Module: crate::imageops::sample
// Provides: {"sinc"}
// Dependencies: {}
fn sinc (t : f32) -> f32 { let a = t * f32 :: consts :: PI ; if t == 0.0 { 1.0 } else { a . sin () / a } }
};
}
