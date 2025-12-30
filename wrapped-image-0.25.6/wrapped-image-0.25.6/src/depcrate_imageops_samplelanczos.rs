// Generated macro for lanczos (function)
macro_rules! Depcrate_imageops_samplelanczos {
() => {
// Module: crate::imageops::sample
// Provides: {"lanczos"}
// Dependencies: {}
fn lanczos (x : f32 , t : f32) -> f32 { if x . abs () < t { sinc (x) * sinc (x / t) } else { 0.0 } }
};
}
