// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_iter_skipimpl_1038 {
() => {
// Module: crate::iter::skip
// Provides: {"impl_1038"}
// Dependencies: {}
impl < I > ParallelIterator for Skip < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
