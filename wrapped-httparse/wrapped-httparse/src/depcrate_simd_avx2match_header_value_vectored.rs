// Generated macro for match_header_value_vectored (function)
macro_rules! Depcrate_simd_avx2match_header_value_vectored {
() => {
// Module: crate::simd::avx2
// Provides: {"match_header_value_vectored"}
// Dependencies: {}
# [target_feature (enable = "avx2")] pub unsafe fn match_header_value_vectored (bytes : & mut Bytes) { while bytes . as_ref () . len () >= 32 { let advance = match_header_value_char_32_avx (bytes . as_ref ()) ; bytes . advance (advance) ; if advance != 32 { return ; } } super :: swar :: match_header_value_vectored (bytes) }
};
}
