// Generated macro for impl_1706 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1706 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1706"}
// Dependencies: {}
impl < T : RealField + RelativeEq < Epsilon = T > > RelativeEq for DualQuaternion < T > { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . to_vector () . relative_eq (& other . to_vector () , epsilon . clone () , max_relative . clone ()) || self . to_vector () . iter () . zip (other . to_vector () . iter ()) . all (| (a , b) | a . relative_eq (& - b . clone () , epsilon . clone () , max_relative . clone ())) } }
};
}
