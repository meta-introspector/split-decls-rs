// Generated macro for avx512_detected (function)
macro_rules! Depcrateavx512_detected {
() => {
// Module: crate
// Provides: {"avx512_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub fn avx512_detected () -> bool { is_x86_feature_detected ! ("avx512f") && is_x86_feature_detected ! ("avx512vl") }
};
}
