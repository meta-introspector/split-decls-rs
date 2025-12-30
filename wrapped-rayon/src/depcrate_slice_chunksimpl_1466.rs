// Generated macro for impl_1466 (impl)
macro_rules! Depcrate_slice_chunksimpl_1466 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1466"}
// Dependencies: {}
impl < 'data , T : 'data + Sync > Producer for ChunksExactProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: ChunksExact < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks_exact (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = index * self . chunk_size ; let (left , right) = self . slice . split_at (elem_index) ; (ChunksExactProducer { chunk_size : self . chunk_size , slice : left , } , ChunksExactProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
};
}
