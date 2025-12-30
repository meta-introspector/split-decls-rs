// Generated macro for uniform_block (function)
macro_rules! Depcrate_simd_swaruniform_block {
() => {
// Module: crate::simd::swar
// Provides: {"uniform_block"}
// Dependencies: {}
const fn uniform_block (b : u8) -> usize { usize :: from_ne_bytes ([b ; BLOCK_SIZE]) }
};
}
