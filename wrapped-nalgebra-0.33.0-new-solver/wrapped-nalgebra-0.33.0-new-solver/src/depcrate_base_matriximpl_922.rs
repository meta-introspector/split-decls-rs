// Generated macro for impl_922 (impl)
macro_rules! Depcrate_base_matriximpl_922 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_922"}
// Dependencies: {}
impl < T : Scalar , D : Dim , S : RawStorage < T , D , D > > SquareMatrix < T , D , S > { # [doc = " The diagonal of this matrix."] # [inline] # [must_use] pub fn diagonal (& self) -> OVector < T , D > where DefaultAllocator : Allocator < D > , { self . map_diagonal (| e | e) } # [doc = " Apply the given function to this matrix's diagonal and returns it."] # [doc = ""] # [doc = " This is a more efficient version of `self.diagonal().map(f)` since this"] # [doc = " allocates only once."] # [must_use] pub fn map_diagonal < T2 : Scalar > (& self , mut f : impl FnMut (T) -> T2) -> OVector < T2 , D > where DefaultAllocator : Allocator < D > , { assert ! (self . is_square () , "Unable to get the diagonal of a non-square matrix.") ; let dim = self . shape_generic () . 0 ; let mut res = Matrix :: uninit (dim , Const :: < 1 >) ; for i in 0 .. dim . value () { unsafe { * res . vget_unchecked_mut (i) = MaybeUninit :: new (f (self . get_unchecked ((i , i)) . clone ())) ; } } unsafe { res . assume_init () } } # [doc = " Computes a trace of a square matrix, i.e., the sum of its diagonal elements."] # [inline] # [must_use] pub fn trace (& self) -> T where T : Scalar + Zero + ClosedAddAssign , { assert ! (self . is_square () , "Cannot compute the trace of non-square matrix.") ; let dim = self . shape_generic () . 0 ; let mut res = T :: zero () ; for i in 0 .. dim . value () { res += unsafe { self . get_unchecked ((i , i)) . clone () } ; } res } }
};
}
