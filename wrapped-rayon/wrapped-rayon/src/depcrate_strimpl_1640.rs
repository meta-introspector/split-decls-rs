// Generated macro for impl_1640 (impl)
macro_rules! Depcrate_strimpl_1640 {
() => {
// Module: crate::str
// Provides: {"impl_1640"}
// Dependencies: {}
impl < 'ch > ParallelIterator for Bytes < 'ch > { type Item = u8 ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (BytesProducer { chars : self . chars } , consumer) } }
};
}
