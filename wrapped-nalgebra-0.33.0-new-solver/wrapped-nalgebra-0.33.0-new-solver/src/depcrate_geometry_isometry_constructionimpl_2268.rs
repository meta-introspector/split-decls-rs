// Generated macro for impl_2268 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2268 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2268"}
// Dependencies: {}
impl < T : SimdRealField , R : AbstractRotation < T , D > , const D : usize > One for Isometry < T , R , D > where T :: Element : SimdRealField , { # [doc = " Creates a new identity isometry."] # [inline] fn one () -> Self { Self :: identity () } }
};
}
