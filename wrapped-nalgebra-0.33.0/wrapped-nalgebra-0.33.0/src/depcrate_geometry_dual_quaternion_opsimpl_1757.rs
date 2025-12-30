// Generated macro for impl_1757 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1757 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1757"}
// Dependencies: {}
impl < T : SimdRealField > Neg for UnitDualQuaternion < T > where T :: Element : SimdRealField , { type Output = UnitDualQuaternion < T > ; # [inline] fn neg (self) -> Self :: Output { UnitDualQuaternion :: new_unchecked (- self . into_inner ()) } }
};
}
