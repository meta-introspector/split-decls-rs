// Generated macro for match_header_value_vectored (function)
macro_rules! Depcrate_simd_runtimematch_header_value_vectored {
() => {
// Module: crate::simd::runtime
// Provides: {"match_header_value_vectored"}
// Dependencies: {}
pub fn match_header_value_vectored (bytes : & mut Bytes) { unsafe { match get_runtime_feature () { AVX2 => avx2 :: match_header_value_vectored (bytes) , SSE42 => sse42 :: match_header_value_vectored (bytes) , _ => super :: swar :: match_header_value_vectored (bytes) , } } }
};
}
