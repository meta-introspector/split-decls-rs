// Generated macro for impl_1644 (impl)
macro_rules! Depcrate_strimpl_1644 {
() => {
// Module: crate::str
// Provides: {"impl_1644"}
// Dependencies: {}
impl < 'ch > ParallelIterator for EncodeUtf16 < 'ch > { type Item = u16 ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (EncodeUtf16Producer { chars : self . chars } , consumer) } }
};
}
