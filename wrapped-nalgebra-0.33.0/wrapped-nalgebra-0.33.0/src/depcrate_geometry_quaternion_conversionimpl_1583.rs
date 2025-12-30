// Generated macro for impl_1583 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1583 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1583"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitQuaternion < T > > for Matrix3 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitQuaternion < T >) -> Self { q . to_rotation_matrix () . into_inner () } }
};
}
