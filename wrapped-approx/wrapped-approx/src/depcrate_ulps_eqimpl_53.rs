// Generated macro for impl_53 (impl)
macro_rules! Depcrate_ulps_eqimpl_53 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : UlpsEq + ? Sized > UlpsEq for cell :: RefCell < T > { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & cell :: RefCell < T > , epsilon : T :: Epsilon , max_ulps : u32) -> bool { T :: ulps_eq (& self . borrow () , & other . borrow () , epsilon , max_ulps) } }
};
}
