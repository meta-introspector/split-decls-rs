// Generated macro for impl_1599 (impl)
macro_rules! Depcrate_sliceimpl_1599 {
() => {
// Module: crate::slice
// Provides: {"impl_1599"}
// Dependencies: {}
impl < 'data , T , P > ParallelIterator for Split < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Sync , { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
