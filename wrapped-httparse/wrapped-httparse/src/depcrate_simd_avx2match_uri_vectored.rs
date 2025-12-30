// Generated macro for match_uri_vectored (function)
macro_rules! Depcrate_simd_avx2match_uri_vectored {
() => {
// Module: crate::simd::avx2
// Provides: {"match_uri_vectored"}
// Dependencies: {}
# [inline] # [target_feature (enable = "avx2")] pub unsafe fn match_uri_vectored (bytes : & mut Bytes) { while bytes . as_ref () . len () >= 32 { let advance = match_url_char_32_avx (bytes . as_ref ()) ; bytes . advance (advance) ; if advance != 32 { return ; } } super :: swar :: match_uri_vectored (bytes) }
};
}
