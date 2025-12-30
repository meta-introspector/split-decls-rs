// Generated macro for impl_920 (impl)
macro_rules! Depcrate_iter_onceimpl_920 {
() => {
// Module: crate::iter::once
// Provides: {"impl_920"}
// Dependencies: {}
impl < T : Send > ParallelIterator for Once < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (1) } }
};
}
