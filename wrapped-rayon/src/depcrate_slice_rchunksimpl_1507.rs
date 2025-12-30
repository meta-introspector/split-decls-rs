// Generated macro for impl_1507 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1507 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1507"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for RChunksExactMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: RChunksExactMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . rchunks_exact_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = self . slice . len () - index * self . chunk_size ; let (left , right) = self . slice . split_at_mut (elem_index) ; (RChunksExactMutProducer { chunk_size : self . chunk_size , slice : right , } , RChunksExactMutProducer { chunk_size : self . chunk_size , slice : left , } ,) } }
};
}
