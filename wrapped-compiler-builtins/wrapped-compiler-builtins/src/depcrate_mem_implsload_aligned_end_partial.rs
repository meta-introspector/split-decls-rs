// Generated macro for load_aligned_end_partial (function)
macro_rules! Depcrate_mem_implsload_aligned_end_partial {
() => {
// Module: crate::mem::impls
// Provides: {"load_aligned_end_partial"}
// Dependencies: {}
# [doc = " Load `load_sz` many bytes from `src.wrapping_byte_add(WORD_SIZE - load_sz)`. `src` must be"] # [doc = " `usize`-aligned. The bytes are returned as the *last* bytes of the return value, i.e., this acts"] # [doc = " as if we had done a `usize` read from `src`, with the out-of-bounds part filled with 0s."] # [doc = " `load_sz` be strictly less than `WORD_SIZE`."] # [cfg (not (feature = "mem-unaligned"))] # [inline (always)] unsafe fn load_aligned_end_partial (src : * const usize , load_sz : usize) -> usize { debug_assert ! (load_sz < WORD_SIZE) ; const { assert ! (WORD_SIZE <= 8) } ; let mut i = 0 ; let mut out = 0usize ; let src_shifted = src . wrapping_byte_add (WORD_SIZE - load_sz) ; let out_shifted = (& raw mut out) . wrapping_byte_add (WORD_SIZE - load_sz) ; i = load_chunk_aligned :: < u8 > (src_shifted , out_shifted , load_sz , i) ; i = load_chunk_aligned :: < u16 > (src_shifted , out_shifted , load_sz , i) ; i = load_chunk_aligned :: < u32 > (src_shifted , out_shifted , load_sz , i) ; debug_assert ! (i == load_sz) ; out }
};
}
