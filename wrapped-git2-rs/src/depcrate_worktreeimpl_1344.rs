// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_worktreeimpl_1344 {
() => {
// Module: crate::worktree
// Provides: {"impl_1344"}
// Dependencies: {}
impl Drop for Worktree { fn drop (& mut self) { unsafe { raw :: git_worktree_free (self . raw) } } }
};
}
