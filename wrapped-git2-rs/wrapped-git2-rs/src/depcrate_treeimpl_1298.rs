// Generated macro for impl_1298 (impl)
macro_rules! Depcrate_treeimpl_1298 {
() => {
// Module: crate::tree
// Provides: {"impl_1298"}
// Dependencies: {}
impl < 'a > Drop for TreeEntry < 'a > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_tree_entry_free (self . raw) } } } }
};
}
