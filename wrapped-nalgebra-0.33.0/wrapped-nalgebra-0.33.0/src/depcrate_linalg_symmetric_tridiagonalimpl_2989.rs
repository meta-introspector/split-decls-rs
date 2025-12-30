// Generated macro for impl_2989 (impl)
macro_rules! Depcrate_linalg_symmetric_tridiagonalimpl_2989 {
() => {
// Module: crate::linalg::symmetric_tridiagonal
// Provides: {"impl_2989"}
// Dependencies: {}
impl < T : ComplexField , D : DimSub < U1 > > Copy for SymmetricTridiagonal < T , D > where DefaultAllocator : Allocator < D , D > + Allocator < DimDiff < D , U1 > > , OMatrix < T , D , D > : Copy , OVector < T , DimDiff < D , U1 > > : Copy , { }
};
}
