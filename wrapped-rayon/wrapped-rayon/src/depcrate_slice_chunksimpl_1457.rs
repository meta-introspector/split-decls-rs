// Generated macro for impl_1457 (impl)
macro_rules! Depcrate_slice_chunksimpl_1457 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1457"}
// Dependencies: {}
impl < T : Sync > IndexedParallelIterator for Chunks < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () . div_ceil (self . chunk_size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (ChunksProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
