// Generated macro for RChunksExactMut (struct)
macro_rules! Depcrate_slice_rchunksRChunksExactMut {
() => {
// Module: crate::slice::rchunks
// Provides: {"RChunksExactMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunksExactMut < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , rem : & 'data mut [T] , }
};
}
