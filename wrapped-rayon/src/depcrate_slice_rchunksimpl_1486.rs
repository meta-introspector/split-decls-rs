// Generated macro for impl_1486 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1486 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1486"}
// Dependencies: {}
impl < T : Sync > IndexedParallelIterator for RChunks < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () . div_ceil (self . chunk_size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (RChunksProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
