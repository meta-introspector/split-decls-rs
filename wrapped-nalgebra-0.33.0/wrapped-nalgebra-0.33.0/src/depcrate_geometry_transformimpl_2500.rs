// Generated macro for impl_2500 (impl)
macro_rules! Depcrate_geometry_transformimpl_2500 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2500"}
// Dependencies: {}
impl TCategory for TAffine { # [inline] fn has_normalizer () -> bool { false } # [inline] fn check_homogeneous_invariants < T : RealField , D : DimName > (mat : & OMatrix < T , D , D >) -> bool where T :: Epsilon : Clone , DefaultAllocator : Allocator < D , D > , { let last = D :: dim () - 1 ; mat . is_invertible () && mat [(last , last)] == T :: one () && (0 .. last) . all (| i | mat [(last , i)] . is_zero ()) } }
};
}
