// Generated macro for impl_2823 (impl)
macro_rules! Depcrate_linalg_hessenbergimpl_2823 {
() => {
// Module: crate::linalg::hessenberg
// Provides: {"impl_2823"}
// Dependencies: {}
impl < T : ComplexField , D : DimSub < U1 > > Copy for Hessenberg < T , D > where DefaultAllocator : Allocator < D , D > + Allocator < DimDiff < D , U1 > > , OMatrix < T , D , D > : Copy , OVector < T , DimDiff < D , U1 > > : Copy , { }
};
}
