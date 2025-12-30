// Generated macro for match_uri_vectored (function)
macro_rules! Depcrate_simd_sse42match_uri_vectored {
() => {
// Module: crate::simd::sse42
// Provides: {"match_uri_vectored"}
// Dependencies: {}
# [target_feature (enable = "sse4.2")] pub unsafe fn match_uri_vectored (bytes : & mut Bytes) { while bytes . as_ref () . len () >= 16 { let advance = match_url_char_16_sse (bytes . as_ref ()) ; bytes . advance (advance) ; if advance != 16 { return ; } } super :: swar :: match_uri_vectored (bytes) ; }
};
}
