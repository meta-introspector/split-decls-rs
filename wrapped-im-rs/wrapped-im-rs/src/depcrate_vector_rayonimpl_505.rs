// Generated macro for impl_505 (impl)
macro_rules! Depcrate_vector_rayonimpl_505 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_505"}
// Dependencies: {}
impl < 'a , A > Producer for VectorProducer < 'a , A > where A : Clone + Send + Sync + 'a , { type Item = & 'a A ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . focus . into_iter () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . focus . split_at (index) ; (VectorProducer { focus : left } , VectorProducer { focus : right } ,) } }
};
}
