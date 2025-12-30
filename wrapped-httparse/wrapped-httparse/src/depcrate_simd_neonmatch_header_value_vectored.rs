// Generated macro for match_header_value_vectored (function)
macro_rules! Depcrate_simd_neonmatch_header_value_vectored {
() => {
// Module: crate::simd::neon
// Provides: {"match_header_value_vectored"}
// Dependencies: {}
# [inline] pub fn match_header_value_vectored (bytes : & mut Bytes) { while bytes . as_ref () . len () >= 16 { unsafe { let advance = match_header_value_char_16_neon (bytes . as_ref () . as_ptr ()) ; bytes . advance (advance) ; if advance != 16 { return ; } } } super :: swar :: match_header_value_vectored (bytes) ; }
};
}
