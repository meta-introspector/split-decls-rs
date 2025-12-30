// Generated macro for impl_11 (impl)
macro_rules! Depcrate_impls_blanketimpl_11 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : ? Sized + ReadReady > ReadReady for & mut T { # [inline] fn read_ready (& mut self) -> Result < bool , Self :: Error > { T :: read_ready (self) } }
};
}
