// Generated macro for ChunkedBitIter (struct)
macro_rules! Depcrate_bit_setChunkedBitIter {
() => {
// Module: crate::bit_set
// Provides: {"ChunkedBitIter"}
// Dependencies: {}
pub struct ChunkedBitIter < 'a , T : Idx > { bit_set : & 'a ChunkedBitSet < T > , chunk_index : usize , chunk_iter : ChunkIter < 'a > , }
};
}
