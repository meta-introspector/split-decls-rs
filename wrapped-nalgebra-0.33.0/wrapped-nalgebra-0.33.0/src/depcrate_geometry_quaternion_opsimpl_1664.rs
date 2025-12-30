// Generated macro for impl_1664 (impl)
macro_rules! Depcrate_geometry_quaternion_opsimpl_1664 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"impl_1664"}
// Dependencies: {}
impl < T : SimdRealField > Neg for Quaternion < T > where T :: Element : SimdRealField , { type Output = Quaternion < T > ; # [inline] fn neg (self) -> Self :: Output { Self :: Output :: from (- self . coords) } }
};
}
