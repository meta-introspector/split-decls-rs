// Generated macro for impl_1941 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1941 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1941"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitComplex < T > > for Matrix2 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitComplex < T >) -> Self { q . to_rotation_matrix () . into_inner () } }
};
}
