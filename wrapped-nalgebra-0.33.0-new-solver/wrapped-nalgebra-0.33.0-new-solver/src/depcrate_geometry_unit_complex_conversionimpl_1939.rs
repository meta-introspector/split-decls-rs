// Generated macro for impl_1939 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1939 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1939"}
// Dependencies: {}
impl < T : SimdRealField > From < Rotation2 < T > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn from (q : Rotation2 < T >) -> Self { Self :: from_rotation_matrix (& q) } }
};
}
