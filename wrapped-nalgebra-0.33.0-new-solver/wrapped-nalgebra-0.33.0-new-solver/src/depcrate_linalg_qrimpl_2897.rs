// Generated macro for impl_2897 (impl)
macro_rules! Depcrate_linalg_qrimpl_2897 {
() => {
// Module: crate::linalg::qr
// Provides: {"impl_2897"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim > Copy for QR < T , R , C > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , OMatrix < T , R , C > : Copy , OVector < T , DimMinimum < R , C > > : Copy , { }
};
}
