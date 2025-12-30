// Generated macro for impl_1718 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1718 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1718"}
// Dependencies: {}
impl < T : RealField + UlpsEq < Epsilon = T > > UlpsEq for UnitDualQuaternion < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . as_ref () . ulps_eq (other . as_ref () , epsilon , max_ulps) } }
};
}
