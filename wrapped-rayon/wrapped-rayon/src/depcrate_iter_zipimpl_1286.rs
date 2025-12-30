// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_iter_zipimpl_1286 {
() => {
// Module: crate::iter::zip
// Provides: {"impl_1286"}
// Dependencies: {}
impl < A , B > ParallelIterator for Zip < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { type Item = (A :: Item , B :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
