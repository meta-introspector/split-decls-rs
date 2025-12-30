// Generated macro for impl_148 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_148 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_148"}
// Dependencies: {}
impl < T : Send > ParallelIterator for Drain < '_ , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
