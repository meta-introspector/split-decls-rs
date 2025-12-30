// Generated macro for impl_1538 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1538 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1538"}
// Dependencies: {}
impl < T : RealField + AbsDiffEq < Epsilon = T > > AbsDiffEq for UnitQuaternion < T > { type Epsilon = T ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . as_ref () . abs_diff_eq (other . as_ref () , epsilon) } }
};
}
