// Generated macro for impl_507 (impl)
macro_rules! Depcrate_vector_rayonimpl_507 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_507"}
// Dependencies: {}
impl < 'a , A > Producer for VectorMutProducer < 'a , A > where A : Clone + Send + Sync + 'a , { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . focus . into_iter () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . focus . split_at (index) ; (VectorMutProducer { focus : left } , VectorMutProducer { focus : right } ,) } }
};
}
