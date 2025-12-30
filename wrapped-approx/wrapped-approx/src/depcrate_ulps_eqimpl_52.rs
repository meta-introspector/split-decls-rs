// Generated macro for impl_52 (impl)
macro_rules! Depcrate_ulps_eqimpl_52 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_52"}
// Dependencies: {}
impl < T : UlpsEq + Copy > UlpsEq for cell :: Cell < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & cell :: Cell < T > , epsilon : T :: Epsilon , max_ulps : u32) -> bool { T :: ulps_eq (& self . get () , & other . get () , epsilon , max_ulps) } }
};
}
