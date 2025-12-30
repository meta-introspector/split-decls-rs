// Generated macro for impl_1636 (impl)
macro_rules! Depcrate_strimpl_1636 {
() => {
// Module: crate::str
// Provides: {"impl_1636"}
// Dependencies: {}
impl < 'ch > ParallelIterator for CharIndices < 'ch > { type Item = (usize , char) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = CharIndicesProducer { index : 0 , chars : self . chars , } ; bridge_unindexed (producer , consumer) } }
};
}
