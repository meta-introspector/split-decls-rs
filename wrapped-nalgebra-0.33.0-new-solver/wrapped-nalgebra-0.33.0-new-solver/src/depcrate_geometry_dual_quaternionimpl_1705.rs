// Generated macro for impl_1705 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1705 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1705"}
// Dependencies: {}
impl < T : RealField + AbsDiffEq < Epsilon = T > > AbsDiffEq for DualQuaternion < T > { type Epsilon = T ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . to_vector () . abs_diff_eq (& other . to_vector () , epsilon . clone ()) || self . to_vector () . iter () . zip (other . to_vector () . iter ()) . all (| (a , b) | a . abs_diff_eq (& - b . clone () , epsilon . clone ())) } }
};
}
