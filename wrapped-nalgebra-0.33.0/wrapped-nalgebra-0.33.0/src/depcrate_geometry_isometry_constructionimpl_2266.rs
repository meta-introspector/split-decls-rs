// Generated macro for impl_2266 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2266 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2266"}
// Dependencies: {}
impl < T : SimdRealField , R : AbstractRotation < T , D > , const D : usize > Default for Isometry < T , R , D > where T :: Element : SimdRealField , { fn default () -> Self { Self :: identity () } }
};
}
