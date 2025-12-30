// Generated macro for match_uri_vectored (function)
macro_rules! Depcrate_simd_swarmatch_uri_vectored {
() => {
// Module: crate::simd::swar
// Provides: {"match_uri_vectored"}
// Dependencies: {}
# [inline] pub fn match_uri_vectored (bytes : & mut Bytes) { loop { if let Some (bytes8) = bytes . peek_n :: < ByteBlock > (BLOCK_SIZE) { let n = match_uri_char_8_swar (bytes8) ; unsafe { bytes . advance (n) ; } if n == BLOCK_SIZE { continue ; } } if let Some (b) = bytes . peek () { if is_uri_token (b) { unsafe { bytes . advance (1) ; } continue ; } } break ; } }
};
}
