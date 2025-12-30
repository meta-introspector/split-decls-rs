// Generated macro for impl_is_zero (macro)
macro_rules! Depcrate_vec_is_zeroimpl_is_zero {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_is_zero"}
// Dependencies: {}
macro_rules ! impl_is_zero { ($ t : ty , $ is_zero : expr) => { unsafe impl IsZero for $ t { # [inline] fn is_zero (& self) -> bool { $ is_zero (* self) } } } ; }
};
}
