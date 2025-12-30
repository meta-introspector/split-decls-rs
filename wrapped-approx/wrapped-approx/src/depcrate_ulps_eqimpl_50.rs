// Generated macro for impl_50 (impl)
macro_rules! Depcrate_ulps_eqimpl_50 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , T : UlpsEq + ? Sized > UlpsEq for & 'a T { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & & 'a T , epsilon : T :: Epsilon , max_ulps : u32) -> bool { T :: ulps_eq (* self , * other , epsilon , max_ulps) } }
};
}
