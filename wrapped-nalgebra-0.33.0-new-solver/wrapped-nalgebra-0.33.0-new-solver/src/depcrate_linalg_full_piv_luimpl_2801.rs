// Generated macro for impl_2801 (impl)
macro_rules! Depcrate_linalg_full_piv_luimpl_2801 {
() => {
// Module: crate::linalg::full_piv_lu
// Provides: {"impl_2801"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for FullPivLU < T , R , C > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , OMatrix < T , R , C > : Copy , PermutationSequence < DimMinimum < R , C > > : Copy , { }
};
}
