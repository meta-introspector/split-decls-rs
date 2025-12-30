// Generated macro for impl_1700 (impl)
macro_rules! Depcrate_vecimpl_1700 {
() => {
// Module: crate::vec
// Provides: {"impl_1700"}
// Dependencies: {}
impl < T : Send > ParallelIterator for IntoIter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
