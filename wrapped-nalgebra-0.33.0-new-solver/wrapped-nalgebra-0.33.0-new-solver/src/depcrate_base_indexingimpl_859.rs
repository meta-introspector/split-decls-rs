// Generated macro for impl_859 (impl)
macro_rules! Depcrate_base_indexingimpl_859 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_859"}
// Dependencies: {}
impl < 'a , T : 'a , R , C , S > MatrixIndexMut < 'a , T , R , C , S > for (usize , usize) where R : Dim , C : Dim , S : RawStorageMut < T , R , C > , { type OutputMut = & 'a mut T ; # [doc (hidden)] # [inline (always)] unsafe fn get_unchecked_mut (self , matrix : & 'a mut Matrix < T , R , C , S >) -> Self :: OutputMut where S : RawStorageMut < T , R , C > , { let (row , col) = self ; matrix . data . get_unchecked_mut (row , col) } }
};
}
