// Generated macro for impl_2858 (impl)
macro_rules! Depcrate_linalg_luimpl_2858 {
() => {
// Module: crate::linalg::lu
// Provides: {"impl_2858"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for LU < T , R , C > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , OMatrix < T , R , C > : Copy , PermutationSequence < DimMinimum < R , C > > : Copy , { }
};
}
