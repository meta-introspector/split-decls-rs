// Generated macro for impl_1674 (impl)
macro_rules! Depcrate_strimpl_1674 {
() => {
// Module: crate::str
// Provides: {"impl_1674"}
// Dependencies: {}
impl < 'ch , P : Pattern > ParallelIterator for MatchIndices < 'ch , P > { type Item = (usize , & 'ch str) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = MatchIndicesProducer { index : 0 , chars : self . chars , pattern : & self . pattern , } ; bridge_unindexed (producer , consumer) } }
};
}
