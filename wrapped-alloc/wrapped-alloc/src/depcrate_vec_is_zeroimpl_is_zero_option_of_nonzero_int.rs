// Generated macro for impl_is_zero_option_of_nonzero_int (macro)
macro_rules! Depcrate_vec_is_zeroimpl_is_zero_option_of_nonzero_int {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_is_zero_option_of_nonzero_int"}
// Dependencies: {}
macro_rules ! impl_is_zero_option_of_nonzero_int { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for Option < NonZero <$ t >> { # [inline] fn is_zero (& self) -> bool { self . is_none () } }) + } ; }
};
}
