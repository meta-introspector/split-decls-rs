// Generated macro for impl_1670 (impl)
macro_rules! Depcrate_strimpl_1670 {
() => {
// Module: crate::str
// Provides: {"impl_1670"}
// Dependencies: {}
impl < 'ch , P : Pattern > ParallelIterator for Matches < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = MatchesProducer { chars : self . chars , pattern : & self . pattern , } ; bridge_unindexed (producer , consumer) } }
};
}
