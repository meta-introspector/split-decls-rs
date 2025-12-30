// Generated macro for impl_795 (impl)
macro_rules! Depcrate_iter_interleaveimpl_795 {
() => {
// Module: crate::iter::interleave
// Provides: {"impl_795"}
// Dependencies: {}
impl < I , J > ParallelIterator for Interleave < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < I :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
