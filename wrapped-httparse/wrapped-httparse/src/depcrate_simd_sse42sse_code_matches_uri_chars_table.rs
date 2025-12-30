// Generated macro for sse_code_matches_uri_chars_table (function)
macro_rules! Depcrate_simd_sse42sse_code_matches_uri_chars_table {
() => {
// Module: crate::simd::sse42
// Provides: {"sse_code_matches_uri_chars_table"}
// Dependencies: {}
# [test] fn sse_code_matches_uri_chars_table () { if ! is_x86_feature_detected ! ("sse4.2") { return ; } # [allow (clippy :: undocumented_unsafe_blocks)] unsafe { assert ! (byte_is_allowed (b'_' , match_uri_vectored)) ; for (b , allowed) in crate :: URI_MAP . iter () . cloned () . enumerate () { assert_eq ! (byte_is_allowed (b as u8 , match_uri_vectored) , allowed , "byte_is_allowed({:?}) should be {:?}" , b , allowed ,) ; } } }
};
}
