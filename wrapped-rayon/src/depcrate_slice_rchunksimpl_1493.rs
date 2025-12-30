// Generated macro for impl_1493 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1493 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1493"}
// Dependencies: {}
impl < T : Sync > IndexedParallelIterator for RChunksExact < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () / self . chunk_size } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (RChunksExactProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
