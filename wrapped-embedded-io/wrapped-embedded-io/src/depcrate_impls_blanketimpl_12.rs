// Generated macro for impl_12 (impl)
macro_rules! Depcrate_impls_blanketimpl_12 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : ? Sized + WriteReady > WriteReady for & mut T { # [inline] fn write_ready (& mut self) -> Result < bool , Self :: Error > { T :: write_ready (self) } }
};
}
