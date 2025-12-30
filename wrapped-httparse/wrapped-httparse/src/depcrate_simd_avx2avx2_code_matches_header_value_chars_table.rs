// Generated macro for avx2_code_matches_header_value_chars_table (function)
macro_rules! Depcrate_simd_avx2avx2_code_matches_header_value_chars_table {
() => {
// Module: crate::simd::avx2
// Provides: {"avx2_code_matches_header_value_chars_table"}
// Dependencies: {}
# [test] fn avx2_code_matches_header_value_chars_table () { if ! is_x86_feature_detected ! ("avx2") { return ; } # [allow (clippy :: undocumented_unsafe_blocks)] unsafe { assert ! (byte_is_allowed (b'_' , match_header_value_vectored)) ; for (b , allowed) in crate :: HEADER_VALUE_MAP . iter () . cloned () . enumerate () { assert_eq ! (byte_is_allowed (b as u8 , match_header_value_vectored) , allowed , "byte_is_allowed({:?}) should be {:?}" , b , allowed ,) ; } } }
};
}
