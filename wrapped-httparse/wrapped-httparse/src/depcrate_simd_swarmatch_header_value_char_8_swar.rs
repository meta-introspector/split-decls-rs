// Generated macro for match_header_value_char_8_swar (function)
macro_rules! Depcrate_simd_swarmatch_header_value_char_8_swar {
() => {
// Module: crate::simd::swar
// Provides: {"match_header_value_char_8_swar"}
// Dependencies: {}
# [inline] fn match_header_value_char_8_swar (block : ByteBlock) -> usize { const M : u8 = 0x20 ; const BM : usize = uniform_block (M) ; const ONE : usize = uniform_block (0x01) ; const DEL : usize = uniform_block (0x7f) ; const M128 : usize = uniform_block (128) ; let x = usize :: from_ne_bytes (block) ; let lt = x . wrapping_sub (BM) & ! x ; let xor_del = x ^ DEL ; let eq_del = xor_del . wrapping_sub (ONE) & ! xor_del ; offsetnz ((lt | eq_del) & M128) }
};
}
