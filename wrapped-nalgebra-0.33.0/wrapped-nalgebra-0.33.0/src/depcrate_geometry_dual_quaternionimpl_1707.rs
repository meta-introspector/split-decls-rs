// Generated macro for impl_1707 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1707 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1707"}
// Dependencies: {}
impl < T : RealField + UlpsEq < Epsilon = T > > UlpsEq for DualQuaternion < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . to_vector () . ulps_eq (& other . to_vector () , epsilon . clone () , max_ulps) || self . to_vector () . iter () . zip (other . to_vector () . iter ()) . all (| (a , b) | a . ulps_eq (& - b . clone () , epsilon . clone () , max_ulps)) } }
};
}
