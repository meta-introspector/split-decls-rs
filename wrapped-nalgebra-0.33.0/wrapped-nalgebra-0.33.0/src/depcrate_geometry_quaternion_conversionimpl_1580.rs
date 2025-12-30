// Generated macro for impl_1580 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1580 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1580"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitQuaternion < T > > for Matrix4 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitQuaternion < T >) -> Self { q . to_homogeneous () } }
};
}
