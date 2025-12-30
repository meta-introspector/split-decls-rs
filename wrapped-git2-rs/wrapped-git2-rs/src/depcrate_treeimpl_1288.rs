// Generated macro for impl_1288 (impl)
macro_rules! Depcrate_treeimpl_1288 {
() => {
// Module: crate::tree
// Provides: {"impl_1288"}
// Dependencies: {}
impl < 'repo > Drop for Tree < 'repo > { fn drop (& mut self) { unsafe { raw :: git_tree_free (self . raw) } } }
};
}
