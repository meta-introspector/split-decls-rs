// Generated macro for impl_150 (impl)
macro_rules! Depcrate_os_windowsimpl_150 {
() => {
// Module: crate::os::windows
// Provides: {"impl_150"}
// Dependencies: {}
impl Drop for ErrorModeGuard { fn drop (& mut self) { unsafe { SetThreadErrorMode (self . 0 , ptr :: null_mut ()) ; } } }
};
}
