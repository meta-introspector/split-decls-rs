// Generated macro for impl_1652 (impl)
macro_rules! Depcrate_strimpl_1652 {
() => {
// Module: crate::str
// Provides: {"impl_1652"}
// Dependencies: {}
impl < 'ch , P : Pattern > ParallelIterator for SplitInclusive < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . chars , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
