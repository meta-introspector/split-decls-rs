// Generated macro for impl_133 (impl)
macro_rules! Depcrate_windowsimpl_133 {
() => {
// Module: crate::windows
// Provides: {"impl_133"}
// Dependencies: {}
impl Drop for ReadDirectoryChangesWatcher { fn drop (& mut self) { let _ = self . tx . send (Action :: Stop) ; self . wakeup_server () ; } }
};
}
