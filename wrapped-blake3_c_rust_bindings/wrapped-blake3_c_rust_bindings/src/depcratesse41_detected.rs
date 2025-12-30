// Generated macro for sse41_detected (function)
macro_rules! Depcratesse41_detected {
() => {
// Module: crate
// Provides: {"sse41_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub fn sse41_detected () -> bool { is_x86_feature_detected ! ("sse4.1") }
};
}
