// Generated macro for FoldChunks (struct)
macro_rules! Depcrate_iter_fold_chunksFoldChunks {
() => {
// Module: crate::iter::fold_chunks
// Provides: {"FoldChunks"}
// Dependencies: {}
# [doc = " `FoldChunks` is an iterator that groups elements of an underlying iterator and applies a"] # [doc = " function over them, producing a single value for each group."] # [doc = ""] # [doc = " This struct is created by the [`fold_chunks()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`fold_chunks()`]: IndexedParallelIterator::fold_chunks()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FoldChunks < I , ID , F > { base : I , chunk_size : usize , fold_op : F , identity : ID , }
};
}
