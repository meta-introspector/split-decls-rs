// Generated macro for impl_43 (impl)
macro_rules! Depcrate_bit_setimpl_43 {
() => {
// Module: crate::bit_set
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a , T : Idx > Iterator for ChunkedBitIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { loop { match & mut self . chunk_iter { ChunkIter :: Zeros => { } ChunkIter :: Ones (iter) => { if let Some (next) = iter . next () { return Some (T :: new (next + self . chunk_index * CHUNK_BITS)) ; } } ChunkIter :: Mixed (iter) => { if let Some (next) = iter . next () { return Some (T :: new (next + self . chunk_index * CHUNK_BITS)) ; } } ChunkIter :: Finished => return None , } self . chunk_index += 1 ; self . chunk_iter = self . bit_set . chunk_iter (self . chunk_index) ; } } }
};
}
