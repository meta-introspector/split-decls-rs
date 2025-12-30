// Generated macro for impl_1501 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1501 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1501"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for RChunksMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: RChunksMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . rchunks_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = self . slice . len () . saturating_sub (index * self . chunk_size) ; let (left , right) = self . slice . split_at_mut (elem_index) ; (RChunksMutProducer { chunk_size : self . chunk_size , slice : right , } , RChunksMutProducer { chunk_size : self . chunk_size , slice : left , } ,) } }
};
}
