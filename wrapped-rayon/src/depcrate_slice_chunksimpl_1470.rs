// Generated macro for impl_1470 (impl)
macro_rules! Depcrate_slice_chunksimpl_1470 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1470"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for ChunksMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () . div_ceil (self . chunk_size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (ChunksMutProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
