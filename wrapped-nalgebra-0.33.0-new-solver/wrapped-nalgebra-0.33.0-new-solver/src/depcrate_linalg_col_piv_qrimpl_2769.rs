// Generated macro for impl_2769 (impl)
macro_rules! Depcrate_linalg_col_piv_qrimpl_2769 {
() => {
// Module: crate::linalg::col_piv_qr
// Provides: {"impl_2769"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for ColPivQR < T , R , C > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , OMatrix < T , R , C > : Copy , PermutationSequence < DimMinimum < R , C > > : Copy , OVector < T , DimMinimum < R , C > > : Copy , { }
};
}
