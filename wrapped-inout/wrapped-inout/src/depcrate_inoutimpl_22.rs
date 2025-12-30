// Generated macro for impl_22 (impl)
macro_rules! Depcrate_inoutimpl_22 {
() => {
// Module: crate::inout
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : Clone > InOut < '_ , '_ , T > { # [doc = " Clone input value and return it."] # [inline (always)] pub fn clone_in (& self) -> T { unsafe { (* self . in_ptr) . clone () } } }
};
}
