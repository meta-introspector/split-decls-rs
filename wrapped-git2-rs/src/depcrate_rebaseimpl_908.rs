// Generated macro for impl_908 (impl)
macro_rules! Depcrate_rebaseimpl_908 {
() => {
// Module: crate::rebase
// Provides: {"impl_908"}
// Dependencies: {}
impl < 'repo > Drop for Rebase < 'repo > { fn drop (& mut self) { unsafe { raw :: git_rebase_free (self . raw) } } }
};
}
