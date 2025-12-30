// Generated macro for match_uri_vectored (function)
macro_rules! Depcrate_simd_runtimematch_uri_vectored {
() => {
// Module: crate::simd::runtime
// Provides: {"match_uri_vectored"}
// Dependencies: {}
pub fn match_uri_vectored (bytes : & mut Bytes) { unsafe { match get_runtime_feature () { AVX2 => avx2 :: match_uri_vectored (bytes) , SSE42 => sse42 :: match_uri_vectored (bytes) , _ => super :: swar :: match_uri_vectored (bytes) , } } }
};
}
