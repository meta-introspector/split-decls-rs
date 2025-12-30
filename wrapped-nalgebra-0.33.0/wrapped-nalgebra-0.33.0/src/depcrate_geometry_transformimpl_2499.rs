// Generated macro for impl_2499 (impl)
macro_rules! Depcrate_geometry_transformimpl_2499 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2499"}
// Dependencies: {}
impl TCategory for TProjective { # [inline] fn check_homogeneous_invariants < T : RealField , D : DimName > (mat : & OMatrix < T , D , D >) -> bool where T :: Epsilon : Clone , DefaultAllocator : Allocator < D , D > , { mat . is_invertible () } }
};
}
