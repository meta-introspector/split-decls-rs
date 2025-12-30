// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1499 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1499"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for RChunksMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () . div_ceil (self . chunk_size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (RChunksMutProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
};
}
