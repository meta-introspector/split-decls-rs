// Generated macro for impl_1648 (impl)
macro_rules! Depcrate_strimpl_1648 {
() => {
// Module: crate::str
// Provides: {"impl_1648"}
// Dependencies: {}
impl < 'ch , P : Pattern > ParallelIterator for Split < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . chars , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
