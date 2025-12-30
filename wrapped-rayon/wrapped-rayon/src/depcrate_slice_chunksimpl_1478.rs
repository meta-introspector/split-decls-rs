// Generated macro for impl_1478 (impl)
macro_rules! Depcrate_slice_chunksimpl_1478 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1478"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for ChunksExactMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: ChunksExactMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks_exact_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = index * self . chunk_size ; let (left , right) = self . slice . split_at_mut (elem_index) ; (ChunksExactMutProducer { chunk_size : self . chunk_size , slice : left , } , ChunksExactMutProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
};
}
