// Generated macro for impl_2977 (impl)
macro_rules! Depcrate_linalg_symmetric_eigenimpl_2977 {
() => {
// Module: crate::linalg::symmetric_eigen
// Provides: {"impl_2977"}
// Dependencies: {}
impl < T : ComplexField , D : DimSub < U1 > , S : Storage < T , D , D > > SquareMatrix < T , D , S > where DefaultAllocator : Allocator < D , D > + Allocator < DimDiff < D , U1 > > + Allocator < D > + Allocator < DimDiff < D , U1 > > , { # [doc = " Computes the eigenvalues of this symmetric matrix."] # [doc = ""] # [doc = " Only the lower-triangular part of the matrix is read."] # [must_use] pub fn symmetric_eigenvalues (& self) -> OVector < T :: RealField , D > { SymmetricEigen :: do_decompose (self . clone_owned () , false , T :: RealField :: default_epsilon () , 0 ,) . unwrap () . 0 } }
};
}
