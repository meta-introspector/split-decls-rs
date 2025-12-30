// Generated macro for impl_770 (impl)
macro_rules! Depcrate_odbimpl_770 {
() => {
// Module: crate::odb
// Provides: {"impl_770"}
// Dependencies: {}
impl < 'repo > Drop for OdbReader < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_stream_free (self . raw) } } }
};
}
