// Generated macro for impl_1182 (impl)
macro_rules! Depcrate_base_par_iterimpl_1182 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1182"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > Producer for ColumnProducerMut < 'a , T , R , C , S > where T : Send + Sync + Scalar , S : Send + Sync , { type Item = MatrixViewMut < 'a , T , R , U1 , S :: RStride , S :: CStride > ; type IntoIter = ColumnIterMut < 'a , T , R , C , S > ; fn into_iter (self) -> Self :: IntoIter { self . 0 } fn split_at (self , index : usize) -> (Self , Self) { let (left_iter , right_iter) = self . 0 . split_at (index) ; (Self (left_iter) , Self (right_iter)) } }
};
}
