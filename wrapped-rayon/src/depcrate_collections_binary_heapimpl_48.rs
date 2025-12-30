// Generated macro for impl_48 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_48 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : Ord + Send > ParallelIterator for Drain < '_ , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
