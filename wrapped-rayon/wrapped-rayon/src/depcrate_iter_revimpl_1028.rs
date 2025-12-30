// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_iter_revimpl_1028 {
() => {
// Module: crate::iter::rev
// Provides: {"impl_1028"}
// Dependencies: {}
impl < I > ParallelIterator for Rev < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
