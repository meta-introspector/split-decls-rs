// Generated macro for impl_57 (impl)
macro_rules! Depcrate_handleimpl_57 {
() => {
// Module: crate::handle
// Provides: {"impl_57"}
// Dependencies: {}
impl Drop for Inner { fn drop (& mut self) { if self . is_exclusive { assert ! (unsafe { CloseHandle (self . handle) != 0 } , "failed to close handle") } } }
};
}
