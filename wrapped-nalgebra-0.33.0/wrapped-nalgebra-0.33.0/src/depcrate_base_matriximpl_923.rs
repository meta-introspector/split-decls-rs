// Generated macro for impl_923 (impl)
macro_rules! Depcrate_base_matriximpl_923 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_923"}
// Dependencies: {}
impl < T : SimdComplexField , D : Dim , S : Storage < T , D , D > > SquareMatrix < T , D , S > { # [doc = " The symmetric part of `self`, i.e., `0.5 * (self + self.transpose())`."] # [inline] # [must_use] pub fn symmetric_part (& self) -> OMatrix < T , D , D > where DefaultAllocator : Allocator < D , D > , { assert ! (self . is_square () , "Cannot compute the symmetric part of a non-square matrix.") ; let mut tr = self . transpose () ; tr += self ; tr *= crate :: convert :: < _ , T > (0.5) ; tr } # [doc = " The hermitian part of `self`, i.e., `0.5 * (self + self.adjoint())`."] # [inline] # [must_use] pub fn hermitian_part (& self) -> OMatrix < T , D , D > where DefaultAllocator : Allocator < D , D > , { assert ! (self . is_square () , "Cannot compute the hermitian part of a non-square matrix.") ; let mut tr = self . adjoint () ; tr += self ; tr *= crate :: convert :: < _ , T > (0.5) ; tr } }
};
}
