// Generated macro for impl_1528 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1528 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1528"}
// Dependencies: {}
impl < T : RealField + RelativeEq < Epsilon = T > > RelativeEq for Quaternion < T > { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . as_vector () . relative_eq (other . as_vector () , epsilon . clone () , max_relative . clone ()) || self . as_vector () . iter () . zip (other . as_vector () . iter ()) . all (| (a , b) | a . relative_eq (& - b . clone () , epsilon . clone () , max_relative . clone ())) } }
};
}
