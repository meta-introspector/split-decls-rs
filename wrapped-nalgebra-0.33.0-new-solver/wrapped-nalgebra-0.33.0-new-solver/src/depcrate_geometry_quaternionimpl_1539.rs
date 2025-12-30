// Generated macro for impl_1539 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1539 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1539"}
// Dependencies: {}
impl < T : RealField + RelativeEq < Epsilon = T > > RelativeEq for UnitQuaternion < T > { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . as_ref () . relative_eq (other . as_ref () , epsilon , max_relative) } }
};
}
