// Generated macro for impl_819 (impl)
macro_rules! Depcrate_base_editionimpl_819 {
() => {
// Module: crate::base::edition
// Provides: {"impl_819"}
// Dependencies: {}
impl < T : Scalar , D : Dim , S : RawStorageMut < T , D , D > > Matrix < T , D , D , S > { # [doc = " Copies the upper-triangle of this matrix to its lower-triangular part."] # [doc = ""] # [doc = " This makes the matrix symmetric. Panics if the matrix is not square."] pub fn fill_lower_triangle_with_upper_triangle (& mut self) { assert ! (self . is_square () , "The input matrix should be square.") ; let dim = self . nrows () ; for j in 0 .. dim { for i in j + 1 .. dim { unsafe { * self . get_unchecked_mut ((i , j)) = self . get_unchecked ((j , i)) . clone () ; } } } } # [doc = " Copies the upper-triangle of this matrix to its upper-triangular part."] # [doc = ""] # [doc = " This makes the matrix symmetric. Panics if the matrix is not square."] pub fn fill_upper_triangle_with_lower_triangle (& mut self) { assert ! (self . is_square () , "The input matrix should be square.") ; for j in 1 .. self . ncols () { for i in 0 .. j { unsafe { * self . get_unchecked_mut ((i , j)) = self . get_unchecked ((j , i)) . clone () ; } } } } }
};
}
