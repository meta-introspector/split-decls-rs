// Generated macro for FoldChunksWith (struct)
macro_rules! Depcrate_iter_fold_chunks_withFoldChunksWith {
() => {
// Module: crate::iter::fold_chunks_with
// Provides: {"FoldChunksWith"}
// Dependencies: {}
# [doc = " `FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a"] # [doc = " function over them, producing a single value for each group."] # [doc = ""] # [doc = " This struct is created by the [`fold_chunks_with()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`fold_chunks_with()`]: IndexedParallelIterator::fold_chunks()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FoldChunksWith < I , U , F > { base : I , chunk_size : usize , item : U , fold_op : F , }
};
}
