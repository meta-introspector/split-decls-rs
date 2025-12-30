// Generated macro for impl_1758 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1758 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1758"}
// Dependencies: {}
impl < 'a , T : SimdRealField > Neg for & 'a UnitDualQuaternion < T > where T :: Element : SimdRealField , { type Output = UnitDualQuaternion < T > ; # [inline] fn neg (self) -> Self :: Output { UnitDualQuaternion :: new_unchecked (- self . as_ref ()) } }
};
}
