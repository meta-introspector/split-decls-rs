// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1938 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1938"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitComplex < T > > for Rotation2 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitComplex < T >) -> Self { q . to_rotation_matrix () } }
};
}
