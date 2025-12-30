// Generated macro for impl_390 (impl)
macro_rules! Depcrate_iter_chunksimpl_390 {
() => {
// Module: crate::iter::chunks
// Provides: {"impl_390"}
// Dependencies: {}
impl < I > ParallelIterator for Chunks < I > where I : IndexedParallelIterator , { type Item = Vec < I :: Item > ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < Vec < I :: Item > > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
