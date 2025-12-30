// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ulps_eqimpl_55 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_55"}
// Dependencies: {}
# [cfg (feature = "num-complex")] impl < T : UlpsEq > UlpsEq for Complex < T > where T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Complex < T > , epsilon : T :: Epsilon , max_ulps : u32) -> bool { T :: ulps_eq (& self . re , & other . re , epsilon . clone () , max_ulps) && T :: ulps_eq (& self . im , & other . im , epsilon , max_ulps) } }
};
}
