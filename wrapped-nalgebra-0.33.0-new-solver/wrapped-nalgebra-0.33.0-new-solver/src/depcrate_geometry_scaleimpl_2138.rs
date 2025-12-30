// Generated macro for impl_2138 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2138 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2138"}
// Dependencies: {}
impl < T : Scalar + UlpsEq , const D : usize > UlpsEq for Scale < T , D > where T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . vector . ulps_eq (& other . vector , epsilon , max_ulps) } }
};
}
