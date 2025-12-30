// Generated macro for impl_713 (impl)
macro_rules! Depcrate_iter_fold_chunksimpl_713 {
() => {
// Module: crate::iter::fold_chunks
// Provides: {"impl_713"}
// Dependencies: {}
impl < I , ID , U , F > ParallelIterator for FoldChunks < I , ID , F > where I : IndexedParallelIterator , ID : Fn () -> U + Send + Sync , F : Fn (U , I :: Item) -> U + Send + Sync , U : Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < U > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
