// Generated macro for impl_ulps_eq (macro)
macro_rules! Depcrate_ulps_eqimpl_ulps_eq {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_ulps_eq"}
// Dependencies: {}
macro_rules ! impl_ulps_eq { ($ T : ident , $ U : ident) => { impl UlpsEq for $ T { # [inline] fn default_max_ulps () -> u32 { 4 } # [inline] fn ulps_eq (& self , other : &$ T , epsilon : $ T , max_ulps : u32) -> bool { if $ T :: abs_diff_eq (self , other , epsilon) { return true ; } if self . signum () != other . signum () { return false ; } let int_self : $ U = self . to_bits () ; let int_other : $ U = other . to_bits () ; if int_self <= int_other { int_other - int_self <= max_ulps as $ U } else { int_self - int_other <= max_ulps as $ U } } } } ; }
};
}
