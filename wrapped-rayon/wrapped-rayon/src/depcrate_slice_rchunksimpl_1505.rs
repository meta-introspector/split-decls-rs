// Generated macro for impl_1505 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1505 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1505"}
// Dependencies: {}
impl < 'data , T : Send + 'data > IndexedParallelIterator for RChunksExactMut < 'data , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () / self . chunk_size } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (RChunksExactMutProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
