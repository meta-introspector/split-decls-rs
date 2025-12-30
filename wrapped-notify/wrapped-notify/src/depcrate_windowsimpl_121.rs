// Generated macro for impl_121 (impl)
macro_rules! Depcrate_windowsimpl_121 {
() => {
// Module: crate::windows
// Provides: {"impl_121"}
// Dependencies: {}
impl ReadDirectoryRequest { fn unwatch (& self) { let _ = self . action_tx . send (Action :: Unwatch (self . data . dir . clone ())) ; } }
};
}
