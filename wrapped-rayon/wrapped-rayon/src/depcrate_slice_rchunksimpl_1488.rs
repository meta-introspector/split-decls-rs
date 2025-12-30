// Generated macro for impl_1488 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1488 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1488"}
// Dependencies: {}
impl < 'data , T : 'data + Sync > Producer for RChunksProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: RChunks < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . rchunks (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = self . slice . len () . saturating_sub (index * self . chunk_size) ; let (left , right) = self . slice . split_at (elem_index) ; (RChunksProducer { chunk_size : self . chunk_size , slice : right , } , RChunksProducer { chunk_size : self . chunk_size , slice : left , } ,) } }
};
}
