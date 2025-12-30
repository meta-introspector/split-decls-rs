// Generated macro for impl_1529 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1529 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1529"}
// Dependencies: {}
impl < T : RealField + UlpsEq < Epsilon = T > > UlpsEq for Quaternion < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . as_vector () . ulps_eq (other . as_vector () , epsilon . clone () , max_ulps) || self . as_vector () . iter () . zip (other . as_vector () . iter ()) . all (| (a , b) | a . ulps_eq (& - b . clone () , epsilon . clone () , max_ulps)) } }
};
}
