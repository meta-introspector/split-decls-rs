// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_iter_repeatimpl_1015 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1015"}
// Dependencies: {}
impl < T > ParallelIterator for RepeatN < T > where T : Clone + Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . inner . len ()) } }
};
}
