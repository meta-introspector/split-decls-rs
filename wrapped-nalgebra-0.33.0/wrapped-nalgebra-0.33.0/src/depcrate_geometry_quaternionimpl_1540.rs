// Generated macro for impl_1540 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1540 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1540"}
// Dependencies: {}
impl < T : RealField + UlpsEq < Epsilon = T > > UlpsEq for UnitQuaternion < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . as_ref () . ulps_eq (other . as_ref () , epsilon , max_ulps) } }
};
}
