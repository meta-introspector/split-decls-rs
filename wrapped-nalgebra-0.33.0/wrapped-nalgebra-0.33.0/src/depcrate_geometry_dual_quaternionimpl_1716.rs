// Generated macro for impl_1716 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1716 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1716"}
// Dependencies: {}
impl < T : RealField + AbsDiffEq < Epsilon = T > > AbsDiffEq for UnitDualQuaternion < T > { type Epsilon = T ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . as_ref () . abs_diff_eq (other . as_ref () , epsilon) } }
};
}
