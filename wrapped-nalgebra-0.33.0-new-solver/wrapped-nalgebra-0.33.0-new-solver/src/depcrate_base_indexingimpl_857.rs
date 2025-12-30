// Generated macro for impl_857 (impl)
macro_rules! Depcrate_base_indexingimpl_857 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_857"}
// Dependencies: {}
impl < 'a , T , R , C , S > MatrixIndexMut < 'a , T , R , C , S > for usize where T : Scalar , R : Dim , C : Dim , S : RawStorageMut < T , R , C > , { type OutputMut = & 'a mut T ; # [doc (hidden)] # [inline (always)] unsafe fn get_unchecked_mut (self , matrix : & 'a mut Matrix < T , R , C , S >) -> Self :: OutputMut where S : RawStorageMut < T , R , C > , { let nrows = matrix . shape () . 0 ; let row = self % nrows ; let col = self / nrows ; matrix . data . get_unchecked_mut (row , col) } }
};
}
