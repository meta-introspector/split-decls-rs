// Generated macro for impl_809 (impl)
macro_rules! Depcrate_iter_interleave_shortestimpl_809 {
() => {
// Module: crate::iter::interleave_shortest
// Provides: {"impl_809"}
// Dependencies: {}
impl < I , J > ParallelIterator for InterleaveShortest < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < I :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
