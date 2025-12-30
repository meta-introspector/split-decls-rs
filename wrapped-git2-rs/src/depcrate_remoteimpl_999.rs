// Generated macro for impl_999 (impl)
macro_rules! Depcrate_remoteimpl_999 {
() => {
// Module: crate::remote
// Provides: {"impl_999"}
// Dependencies: {}
impl < 'repo > Drop for Remote < 'repo > { fn drop (& mut self) { unsafe { raw :: git_remote_free (self . raw) } } }
};
}
