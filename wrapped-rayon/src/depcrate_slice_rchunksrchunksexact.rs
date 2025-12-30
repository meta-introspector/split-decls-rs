// Generated macro for RChunksExact (struct)
macro_rules! Depcrate_slice_rchunksRChunksExact {
() => {
// Module: crate::slice::rchunks
// Provides: {"RChunksExact"}
// Dependencies: {}
# [doc = " Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunksExact < 'data , T > { chunk_size : usize , slice : & 'data [T] , rem : & 'data [T] , }
};
}
