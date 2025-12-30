// Generated macro for impl_1603 (impl)
macro_rules! Depcrate_sliceimpl_1603 {
() => {
// Module: crate::slice
// Provides: {"impl_1603"}
// Dependencies: {}
impl < 'data , T , P > ParallelIterator for SplitInclusive < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Sync , { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
