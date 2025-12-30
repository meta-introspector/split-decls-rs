// Generated macro for sse2_detected (function)
macro_rules! Depcratesse2_detected {
() => {
// Module: crate
// Provides: {"sse2_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub fn sse2_detected () -> bool { is_x86_feature_detected ! ("sse2") }
};
}
