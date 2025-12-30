// Generated macro for ChunksExact (struct)
macro_rules! Depcrate_slice_chunksChunksExact {
() => {
// Module: crate::slice::chunks
// Provides: {"ChunksExact"}
// Dependencies: {}
# [doc = " Parallel iterator over immutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct ChunksExact < 'data , T > { chunk_size : usize , slice : & 'data [T] , rem : & 'data [T] , }
};
}
