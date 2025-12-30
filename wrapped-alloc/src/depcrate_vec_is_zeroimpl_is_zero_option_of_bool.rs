// Generated macro for impl_is_zero_option_of_bool (macro)
macro_rules! Depcrate_vec_is_zeroimpl_is_zero_option_of_bool {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_is_zero_option_of_bool"}
// Dependencies: {}
macro_rules ! impl_is_zero_option_of_bool { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for $ t { # [inline] fn is_zero (& self) -> bool { let raw : u8 = unsafe { core :: mem :: transmute (* self) } ; raw == 0 } }) + } ; }
};
}
