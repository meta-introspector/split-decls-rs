// Generated macro for impl_1449 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1449 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1449"}
// Dependencies: {}
impl < 'data , T , P > ParallelIterator for ChunkByMut < 'data , T , P > where T : Send , P : Fn (& T , & T) -> bool + Send + Sync , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (ChunkByProducer { tail : self . slice . len () , slice : self . slice , pred : & self . pred , marker : PhantomData , } , consumer ,) } }
};
}
