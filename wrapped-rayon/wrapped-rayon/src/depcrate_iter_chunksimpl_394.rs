// Generated macro for impl_394 (impl)
macro_rules! Depcrate_iter_chunksimpl_394 {
() => {
// Module: crate::iter::chunks
// Provides: {"impl_394"}
// Dependencies: {}
impl < P , F , T > Producer for ChunkProducer < P , F > where P : Producer , F : Fn (P :: IntoIter) -> T + Send + Clone , { type Item = T ; type IntoIter = std :: iter :: Map < ChunkSeq < P > , F > ; fn into_iter (self) -> Self :: IntoIter { let chunks = ChunkSeq { chunk_size : self . chunk_size , len : self . len , inner : if self . len > 0 { Some (self . base) } else { None } , } ; chunks . map (self . map) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = Ord :: min (index * self . chunk_size , self . len) ; let (left , right) = self . base . split_at (elem_index) ; (ChunkProducer { chunk_size : self . chunk_size , len : elem_index , base : left , map : self . map . clone () , } , ChunkProducer { chunk_size : self . chunk_size , len : self . len - elem_index , base : right , map : self . map , } ,) } fn min_len (& self) -> usize { self . base . min_len () . div_ceil (self . chunk_size) } fn max_len (& self) -> usize { self . base . max_len () / self . chunk_size } }
};
}
