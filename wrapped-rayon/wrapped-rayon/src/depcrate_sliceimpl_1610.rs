// Generated macro for impl_1610 (impl)
macro_rules! Depcrate_sliceimpl_1610 {
() => {
// Module: crate::slice
// Provides: {"impl_1610"}
// Dependencies: {}
impl < 'data , T , P > ParallelIterator for SplitInclusiveMut < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
