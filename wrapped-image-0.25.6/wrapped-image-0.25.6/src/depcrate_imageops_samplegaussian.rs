// Generated macro for gaussian (function)
macro_rules! Depcrate_imageops_samplegaussian {
() => {
// Module: crate::imageops::sample
// Provides: {"gaussian"}
// Dependencies: {}
# [doc = " The Gaussian Function."] # [doc = " ```r``` is the standard deviation."] pub (crate) fn gaussian (x : f32 , r : f32) -> f32 { ((2.0 * f32 :: consts :: PI) . sqrt () * r) . recip () * (- x . powi (2) / (2.0 * r . powi (2))) . exp () }
};
}
