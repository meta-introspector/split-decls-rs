// Generated macro for impl_724 (impl)
macro_rules! Depcrate_iter_fold_chunks_withimpl_724 {
() => {
// Module: crate::iter::fold_chunks_with
// Provides: {"impl_724"}
// Dependencies: {}
impl < I , U , F > ParallelIterator for FoldChunksWith < I , U , F > where I : IndexedParallelIterator , U : Send + Clone , F : Fn (U , I :: Item) -> U + Send + Sync , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < U > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
