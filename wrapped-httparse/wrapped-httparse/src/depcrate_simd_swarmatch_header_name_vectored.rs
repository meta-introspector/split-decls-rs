// Generated macro for match_header_name_vectored (function)
macro_rules! Depcrate_simd_swarmatch_header_name_vectored {
() => {
// Module: crate::simd::swar
// Provides: {"match_header_name_vectored"}
// Dependencies: {}
# [inline] pub fn match_header_name_vectored (bytes : & mut Bytes) { while let Some (block) = bytes . peek_n :: < ByteBlock > (BLOCK_SIZE) { let n = match_block (is_header_name_token , block) ; unsafe { bytes . advance (n) ; } if n != BLOCK_SIZE { return ; } } unsafe { bytes . advance (match_tail (is_header_name_token , bytes . as_ref ())) } ; }
};
}
