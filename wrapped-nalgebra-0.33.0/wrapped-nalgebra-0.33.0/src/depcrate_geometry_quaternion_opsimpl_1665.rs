// Generated macro for impl_1665 (impl)
macro_rules! Depcrate_geometry_quaternion_opsimpl_1665 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"impl_1665"}
// Dependencies: {}
impl < 'a , T : SimdRealField > Neg for & 'a Quaternion < T > where T :: Element : SimdRealField , { type Output = Quaternion < T > ; # [inline] fn neg (self) -> Self :: Output { Self :: Output :: from (- & self . coords) } }
};
}
