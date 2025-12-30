// Generated macro for impl_is_zero_option_of_int (macro)
macro_rules! Depcrate_vec_is_zeroimpl_is_zero_option_of_int {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_is_zero_option_of_int"}
// Dependencies: {}
macro_rules ! impl_is_zero_option_of_int { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for Option <$ t > { # [inline] fn is_zero (& self) -> bool { const { let none : Self = unsafe { core :: mem :: MaybeUninit :: zeroed () . assume_init () } ; assert ! (none . is_none ()) ; } self . is_none () } }) + } ; }
};
}
