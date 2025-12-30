// Generated macro for impl_207 (impl)
macro_rules! Depcrate_processimpl_207 {
() => {
// Module: crate::process
// Provides: {"impl_207"}
// Dependencies: {}
impl < T > NonBlocking for & mut T where T : NonBlocking , { fn set_blocking (& mut self , on : bool) -> Result < () > { T :: set_blocking (self , on) } }
};
}
