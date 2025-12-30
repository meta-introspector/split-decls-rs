// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_iter_repeatimpl_1008 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1008"}
// Dependencies: {}
impl < T > ParallelIterator for Repeat < T > where T : Clone + Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = RepeatProducer { element : self . element , } ; bridge_unindexed (producer , consumer) } }
};
}
