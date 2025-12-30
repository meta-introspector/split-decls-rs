// Generated macro for impl_241 (impl)
macro_rules! Depcrate_windows_termimpl_241 {
() => {
// Module: crate::windows_term
// Provides: {"impl_241"}
// Dependencies: {}
impl ConsoleModeGuard { fn set (handle : HANDLE , mode : CONSOLE_MODE , enable : bool) -> Option < Self > { Some (ConsoleModeGuard { handle , restore_mode : set_console_mode (handle , mode , enable) ? , }) } }
};
}
