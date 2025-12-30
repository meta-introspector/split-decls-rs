// Generated macro for impl_1657 (impl)
macro_rules! Depcrate_strimpl_1657 {
() => {
// Module: crate::str
// Provides: {"impl_1657"}
// Dependencies: {}
impl < 'ch , P : Pattern > ParallelIterator for SplitTerminator < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitTerminatorProducer :: new (self . chars , & self . terminator) ; bridge_unindexed (producer , consumer) } }
};
}
