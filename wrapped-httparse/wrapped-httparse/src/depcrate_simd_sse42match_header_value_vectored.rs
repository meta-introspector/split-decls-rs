// Generated macro for match_header_value_vectored (function)
macro_rules! Depcrate_simd_sse42match_header_value_vectored {
() => {
// Module: crate::simd::sse42
// Provides: {"match_header_value_vectored"}
// Dependencies: {}
# [target_feature (enable = "sse4.2")] pub unsafe fn match_header_value_vectored (bytes : & mut Bytes) { while bytes . as_ref () . len () >= 16 { let advance = match_header_value_char_16_sse (bytes . as_ref ()) ; bytes . advance (advance) ; if advance != 16 { return ; } } super :: swar :: match_header_value_vectored (bytes) ; }
};
}
