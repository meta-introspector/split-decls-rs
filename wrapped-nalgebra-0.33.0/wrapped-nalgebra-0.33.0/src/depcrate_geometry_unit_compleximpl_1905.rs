// Generated macro for impl_1905 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1905 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1905"}
// Dependencies: {}
impl < T : RealField > UlpsEq for UnitComplex < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . re . ulps_eq (& other . re , epsilon . clone () , max_ulps) && self . im . ulps_eq (& other . im , epsilon , max_ulps) } }
};
}
