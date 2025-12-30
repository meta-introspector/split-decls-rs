// Generated macro for impl_1940 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1940 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1940"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitComplex < T > > for Matrix3 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitComplex < T >) -> Matrix3 < T > { q . to_homogeneous () } }
};
}
