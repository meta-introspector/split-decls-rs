// Generated macro for impl_1527 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1527 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1527"}
// Dependencies: {}
impl < T : RealField + AbsDiffEq < Epsilon = T > > AbsDiffEq for Quaternion < T > { type Epsilon = T ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . as_vector () . abs_diff_eq (other . as_vector () , epsilon . clone ()) || self . as_vector () . iter () . zip (other . as_vector () . iter ()) . all (| (a , b) | a . abs_diff_eq (& - b . clone () , epsilon . clone ())) } }
};
}
