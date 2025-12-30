// Generated macro for load_aligned_partial (function)
macro_rules! Depcrate_mem_implsload_aligned_partial {
() => {
// Module: crate::mem::impls
// Provides: {"load_aligned_partial"}
// Dependencies: {}
# [doc = " Load `load_sz` many bytes from `src`, which must be usize-aligned. Acts as if we did a `usize`"] # [doc = " read with the out-of-bounds part filled with 0s."] # [doc = " `load_sz` be strictly less than `WORD_SIZE`."] # [cfg (not (feature = "mem-unaligned"))] # [inline (always)] unsafe fn load_aligned_partial (src : * const usize , load_sz : usize) -> usize { debug_assert ! (load_sz < WORD_SIZE) ; const { assert ! (WORD_SIZE <= 8) } ; let mut i = 0 ; let mut out = 0usize ; i = load_chunk_aligned :: < u32 > (src , & raw mut out , load_sz , i) ; i = load_chunk_aligned :: < u16 > (src , & raw mut out , load_sz , i) ; i = load_chunk_aligned :: < u8 > (src , & raw mut out , load_sz , i) ; debug_assert ! (i == load_sz) ; out }
};
}
