// Generated macro for impl_1709 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1709 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1709"}
// Dependencies: {}
impl < T : Scalar + ClosedNeg + PartialEq + SimdRealField > PartialEq for UnitDualQuaternion < T > { # [inline] fn eq (& self , rhs : & Self) -> bool { self . as_ref () . eq (rhs . as_ref ()) } }
};
}
