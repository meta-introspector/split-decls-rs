// Generated macro for impl_196 (impl)
macro_rules! Depcrate_process_windowsimpl_196 {
() => {
// Module: crate::process::windows
// Provides: {"impl_196"}
// Dependencies: {}
impl NonBlocking for ProcessStream { fn set_blocking (& mut self , on : bool) -> Result < () > { self . output . blocking (on) ; Ok (()) } }
};
}
