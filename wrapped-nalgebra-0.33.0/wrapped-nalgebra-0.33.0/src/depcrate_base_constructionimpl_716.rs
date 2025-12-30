// Generated macro for impl_716 (impl)
macro_rules! Depcrate_base_constructionimpl_716 {
() => {
// Module: crate::base::construction
// Provides: {"impl_716"}
// Dependencies: {}
impl < T , D : DimName > One for OMatrix < T , D , D > where T : Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign , DefaultAllocator : Allocator < D , D > , { # [inline] fn one () -> Self { Self :: identity () } }
};
}
