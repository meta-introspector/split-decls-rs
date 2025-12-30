// Generated macro for avx2_detected (function)
macro_rules! Depcrateavx2_detected {
() => {
// Module: crate
// Provides: {"avx2_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub fn avx2_detected () -> bool { is_x86_feature_detected ! ("avx2") }
};
}
