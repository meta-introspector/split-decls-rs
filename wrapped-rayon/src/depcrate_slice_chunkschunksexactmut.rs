// Generated macro for ChunksExactMut (struct)
macro_rules! Depcrate_slice_chunksChunksExactMut {
() => {
// Module: crate::slice::chunks
// Provides: {"ChunksExactMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct ChunksExactMut < 'data , T > { chunk_size : usize , slice : & 'data mut [T] , rem : & 'data mut [T] , }
};
}
