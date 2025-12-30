// Generated macro for impl_426 (impl)
macro_rules! Depcrate_commitimpl_426 {
() => {
// Module: crate::commit
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'repo > Drop for Commit < 'repo > { fn drop (& mut self) { unsafe { raw :: git_commit_free (self . raw) } } }
};
}
