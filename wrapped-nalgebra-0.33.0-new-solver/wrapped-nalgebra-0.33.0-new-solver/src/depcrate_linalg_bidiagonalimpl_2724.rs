// Generated macro for impl_2724 (impl)
macro_rules! Depcrate_linalg_bidiagonalimpl_2724 {
() => {
// Module: crate::linalg::bidiagonal
// Provides: {"impl_2724"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for Bidiagonal < T , R , C > where DimMinimum < R , C > : DimSub < U1 > , DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > + Allocator < DimDiff < DimMinimum < R , C > , U1 > > , OMatrix < T , R , C > : Copy , OVector < T , DimMinimum < R , C > > : Copy , OVector < T , DimDiff < DimMinimum < R , C > , U1 > > : Copy , { }
};
}
