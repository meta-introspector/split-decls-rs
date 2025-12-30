// Generated macro for impl_1343 (impl)
macro_rules! Depcrate_worktreeimpl_1343 {
() => {
// Module: crate::worktree
// Provides: {"impl_1343"}
// Dependencies: {}
impl Binding for Worktree { type Raw = * mut raw :: git_worktree ; unsafe fn from_raw (ptr : * mut raw :: git_worktree) -> Worktree { Worktree { raw : ptr } } fn raw (& self) -> * mut raw :: git_worktree { self . raw } }
};
}
