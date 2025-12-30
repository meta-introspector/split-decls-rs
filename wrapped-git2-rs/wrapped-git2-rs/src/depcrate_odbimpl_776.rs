// Generated macro for impl_776 (impl)
macro_rules! Depcrate_odbimpl_776 {
() => {
// Module: crate::odb
// Provides: {"impl_776"}
// Dependencies: {}
impl < 'repo > Drop for OdbWriter < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_stream_free (self . raw) } } }
};
}
