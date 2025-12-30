// Generated macro for impl_51 (impl)
macro_rules! Depcrate_ulps_eqimpl_51 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , T : UlpsEq + ? Sized > UlpsEq for & 'a mut T { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & & 'a mut T , epsilon : T :: Epsilon , max_ulps : u32) -> bool { T :: ulps_eq (* self , * other , epsilon , max_ulps) } }
};
}
