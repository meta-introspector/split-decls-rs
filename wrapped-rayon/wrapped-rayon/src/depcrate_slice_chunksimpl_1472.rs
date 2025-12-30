// Generated macro for impl_1472 (impl)
macro_rules! Depcrate_slice_chunksimpl_1472 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1472"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for ChunksMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: ChunksMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = Ord :: min (index * self . chunk_size , self . slice . len ()) ; let (left , right) = self . slice . split_at_mut (elem_index) ; (ChunksMutProducer { chunk_size : self . chunk_size , slice : left , } , ChunksMutProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
};
}
