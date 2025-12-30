// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1717 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1717"}
// Dependencies: {}
impl < T : RealField + RelativeEq < Epsilon = T > > RelativeEq for UnitDualQuaternion < T > { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . as_ref () . relative_eq (other . as_ref () , epsilon , max_relative) } }
};
}
