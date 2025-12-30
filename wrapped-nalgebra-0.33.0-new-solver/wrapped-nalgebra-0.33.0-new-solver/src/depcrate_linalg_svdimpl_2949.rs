// Generated macro for impl_2949 (impl)
macro_rules! Depcrate_linalg_svdimpl_2949 {
() => {
// Module: crate::linalg::svd
// Provides: {"impl_2949"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for SVD < T , R , C > where DefaultAllocator : Allocator < DimMinimum < R , C > , C > + Allocator < R , DimMinimum < R , C > > + Allocator < DimMinimum < R , C > > , OMatrix < T , R , DimMinimum < R , C > > : Copy , OMatrix < T , DimMinimum < R , C > , C > : Copy , OVector < T :: RealField , DimMinimum < R , C > > : Copy , { }
};
}
