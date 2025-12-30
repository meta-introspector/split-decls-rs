// Generated macro for impl_1582 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1582 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1582"}
// Dependencies: {}
impl < T : SimdRealField > From < Rotation3 < T > > for UnitQuaternion < T > where T :: Element : SimdRealField , { # [inline] fn from (q : Rotation3 < T >) -> Self { Self :: from_rotation_matrix (& q) } }
};
}
