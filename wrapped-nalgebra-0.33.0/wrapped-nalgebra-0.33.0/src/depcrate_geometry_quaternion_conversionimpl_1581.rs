// Generated macro for impl_1581 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1581 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1581"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitQuaternion < T > > for Rotation3 < T > where T :: Element : SimdRealField , { # [inline] fn from (q : UnitQuaternion < T >) -> Self { q . to_rotation_matrix () } }
};
}
