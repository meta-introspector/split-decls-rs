// Generated macro for impl_1607 (impl)
macro_rules! Depcrate_sliceimpl_1607 {
() => {
// Module: crate::slice
// Provides: {"impl_1607"}
// Dependencies: {}
impl < 'data , T , P > ParallelIterator for SplitMut < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
};
}
