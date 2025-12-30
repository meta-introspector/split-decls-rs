// Generated macro for ChunkByMut (struct)
macro_rules! Depcrate_slice_chunk_byChunkByMut {
() => {
// Module: crate::slice::chunk_by
// Provides: {"ChunkByMut"}
// Dependencies: {}
# [doc = " Parallel iterator over slice in (non-overlapping) mutable chunks"] # [doc = " separated by a predicate."] # [doc = ""] # [doc = " This struct is created by the [`par_chunk_by_mut`] method on `&mut [T]`."] # [doc = ""] # [doc = " [`par_chunk_by_mut`]: super::ParallelSliceMut::par_chunk_by_mut()"] pub struct ChunkByMut < 'data , T , P > { pred : P , slice : & 'data mut [T] , }
};
}
