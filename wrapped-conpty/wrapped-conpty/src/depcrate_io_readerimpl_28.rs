// Generated macro for impl_28 (impl)
macro_rules! Depcrate_io_readerimpl_28 {
() => {
// Module: crate::io::reader
// Provides: {"impl_28"}
// Dependencies: {}
impl Drop for PipeReader { fn drop (& mut self) { unsafe { CloseHandle (self . handle) . ok () . unwrap () ; } } }
};
}
