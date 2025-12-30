// Generated macro for impl_956 (impl)
macro_rules! Depcrate_reflogimpl_956 {
() => {
// Module: crate::reflog
// Provides: {"impl_956"}
// Dependencies: {}
impl Drop for Reflog { fn drop (& mut self) { unsafe { raw :: git_reflog_free (self . raw) } } }
};
}
