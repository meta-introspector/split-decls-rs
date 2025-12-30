// Generated macro for impl_47 (impl)
macro_rules! Depcrate_myersimpl_47 {
() => {
// Module: crate::myers
// Provides: {"impl_47"}
// Dependencies: {}
impl Drop for Myers { fn drop (& mut self) { unsafe { drop (Box :: from_raw (self . kvec . as_ptr ())) } } }
};
}
