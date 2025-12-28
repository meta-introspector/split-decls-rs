macro_rules! ChunkBy {
    () => {
        # [doc = " Parallel iterator over slice in (non-overlapping) chunks separated by a predicate."] # [doc = ""] # [doc = " This struct is created by the [`par_chunk_by`] method on `&[T]`."] # [doc = ""] # [doc = " [`par_chunk_by`]: super::ParallelSlice::par_chunk_by()"] pub struct ChunkBy < 'data , T , P > { pred : P , slice : & 'data [T] , }
    };
}

ChunkBy!()