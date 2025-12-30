// Generated macro for RChunksMut (struct)
macro_rules! Depcrate_slice_rchunksRChunksMut {
() => {
// Module: crate::slice::rchunks
// Provides: {"RChunksMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunksMut < 'data , T > { chunk_size : usize , slice : & 'data mut [T] , }
};
}
