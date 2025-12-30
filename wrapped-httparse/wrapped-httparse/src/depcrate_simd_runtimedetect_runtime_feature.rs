// Generated macro for detect_runtime_feature (function)
macro_rules! Depcrate_simd_runtimedetect_runtime_feature {
() => {
// Module: crate::simd::runtime
// Provides: {"detect_runtime_feature"}
// Dependencies: {}
fn detect_runtime_feature () -> u8 { if is_x86_feature_detected ! ("avx2") { AVX2 } else if is_x86_feature_detected ! ("sse4.2") { SSE42 } else { NOP } }
};
}
