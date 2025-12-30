// Generated macro for impl_918 (impl)
macro_rules! Depcrate_base_matriximpl_918 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_918"}
// Dependencies: {}
impl < T : Scalar , D : Dim , S : RawStorageMut < T , D , D > > Matrix < T , D , D , S > { # [doc = " Transposes the square matrix `self` in-place."] pub fn transpose_mut (& mut self) { assert ! (self . is_square () , "Unable to transpose a non-square matrix in-place.") ; let dim = self . shape () . 0 ; for i in 1 .. dim { for j in 0 .. i { unsafe { self . swap_unchecked ((i , j) , (j , i)) } } } } }
};
}
