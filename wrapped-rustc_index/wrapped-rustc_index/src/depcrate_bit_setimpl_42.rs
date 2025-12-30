// Generated macro for impl_42 (impl)
macro_rules! Depcrate_bit_setimpl_42 {
() => {
// Module: crate::bit_set
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , T : Idx > ChunkedBitIter < 'a , T > { # [inline] fn new (bit_set : & 'a ChunkedBitSet < T >) -> ChunkedBitIter < 'a , T > { ChunkedBitIter { bit_set , chunk_index : 0 , chunk_iter : bit_set . chunk_iter (0) } } }
};
}
