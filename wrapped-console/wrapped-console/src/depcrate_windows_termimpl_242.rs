// Generated macro for impl_242 (impl)
macro_rules! Depcrate_windows_termimpl_242 {
() => {
// Module: crate::windows_term
// Provides: {"impl_242"}
// Dependencies: {}
impl Drop for ConsoleModeGuard { fn drop (& mut self) { unsafe { SetConsoleMode (self . handle , self . restore_mode) ; } } }
};
}
