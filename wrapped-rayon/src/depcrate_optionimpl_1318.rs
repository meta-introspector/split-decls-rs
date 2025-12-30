// Generated macro for impl_1318 (impl)
macro_rules! Depcrate_optionimpl_1318 {
() => {
// Module: crate::option
// Provides: {"impl_1318"}
// Dependencies: {}
impl < T : Send > ParallelIterator for IntoIter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
