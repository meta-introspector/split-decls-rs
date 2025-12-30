// Generated macro for load_chunk_aligned (function)
macro_rules! Depcrate_mem_implsload_chunk_aligned {
() => {
// Module: crate::mem::impls
// Provides: {"load_chunk_aligned"}
// Dependencies: {}
# [doc = " Loads a `T`-sized chunk from `src` into `dst` at offset `offset`, if that does not exceed"] # [doc = " `load_sz`. The offset pointers must both be `T`-aligned. Returns the new offset, advanced by the"] # [doc = " chunk size if a load happened."] # [cfg (not (feature = "mem-unaligned"))] # [inline (always)] unsafe fn load_chunk_aligned < T : Copy > (src : * const usize , dst : * mut usize , load_sz : usize , offset : usize ,) -> usize { let chunk_sz = core :: mem :: size_of :: < T > () ; if (load_sz & chunk_sz) != 0 { * dst . wrapping_byte_add (offset) . cast :: < T > () = * src . wrapping_byte_add (offset) . cast :: < T > () ; offset | chunk_sz } else { offset } }
};
}
