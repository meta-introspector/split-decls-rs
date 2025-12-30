// Generated macro for impl_494 (impl)
macro_rules! Depcrate_iter_emptyimpl_494 {
() => {
// Module: crate::iter::empty
// Provides: {"impl_494"}
// Dependencies: {}
impl < T : Send > ParallelIterator for Empty < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (0) } }
};
}
