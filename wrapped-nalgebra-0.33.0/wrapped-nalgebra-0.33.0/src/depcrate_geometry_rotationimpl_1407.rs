// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1407 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1407"}
// Dependencies: {}
impl < T , const D : usize > UlpsEq for Rotation < T , D > where T : Scalar + UlpsEq , T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . matrix . ulps_eq (& other . matrix , epsilon , max_ulps) } }
};
}
