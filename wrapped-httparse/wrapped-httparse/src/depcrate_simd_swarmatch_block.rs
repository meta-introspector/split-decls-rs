// Generated macro for match_block (function)
macro_rules! Depcrate_simd_swarmatch_block {
() => {
// Module: crate::simd::swar
// Provides: {"match_block"}
// Dependencies: {}
# [inline (always)] fn match_block (f : impl Fn (u8) -> bool , block : ByteBlock) -> usize { for (i , & b) in block . iter () . enumerate () { if ! f (b) { return i ; } } BLOCK_SIZE }
};
}
