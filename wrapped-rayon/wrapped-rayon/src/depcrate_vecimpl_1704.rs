// Generated macro for impl_1704 (impl)
macro_rules! Depcrate_vecimpl_1704 {
() => {
// Module: crate::vec
// Provides: {"impl_1704"}
// Dependencies: {}
impl < 'data , T : Send > ParallelIterator for Drain < 'data , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
