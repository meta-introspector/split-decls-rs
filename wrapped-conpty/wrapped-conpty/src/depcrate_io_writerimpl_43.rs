// Generated macro for impl_43 (impl)
macro_rules! Depcrate_io_writerimpl_43 {
() => {
// Module: crate::io::writer
// Provides: {"impl_43"}
// Dependencies: {}
impl Drop for PipeWriter { fn drop (& mut self) { unsafe { CloseHandle (self . handle) . ok () . unwrap () ; } } }
};
}
