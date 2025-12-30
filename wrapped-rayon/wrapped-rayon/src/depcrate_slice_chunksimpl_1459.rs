// Generated macro for impl_1459 (impl)
macro_rules! Depcrate_slice_chunksimpl_1459 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1459"}
// Dependencies: {}
impl < 'data , T : 'data + Sync > Producer for ChunksProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: Chunks < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = Ord :: min (index * self . chunk_size , self . slice . len ()) ; let (left , right) = self . slice . split_at (elem_index) ; (ChunksProducer { chunk_size : self . chunk_size , slice : left , } , ChunksProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
};
}
