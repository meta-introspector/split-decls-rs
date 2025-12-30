// Generated macro for ChunkByProducer (struct)
macro_rules! Depcrate_slice_chunk_byChunkByProducer {
() => {
// Module: crate::slice::chunk_by
// Provides: {"ChunkByProducer"}
// Dependencies: {}
struct ChunkByProducer < 'p , T , Slice , Pred > { slice : Slice , pred : & 'p Pred , tail : usize , marker : PhantomData < fn (& T) > , }
};
}
