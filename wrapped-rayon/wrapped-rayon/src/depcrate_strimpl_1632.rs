// Generated macro for impl_1632 (impl)
macro_rules! Depcrate_strimpl_1632 {
() => {
// Module: crate::str
// Provides: {"impl_1632"}
// Dependencies: {}
impl < 'ch > ParallelIterator for Chars < 'ch > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (CharsProducer { chars : self . chars } , consumer) } }
};
}
