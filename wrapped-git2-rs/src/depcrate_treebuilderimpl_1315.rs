// Generated macro for impl_1315 (impl)
macro_rules! Depcrate_treebuilderimpl_1315 {
() => {
// Module: crate::treebuilder
// Provides: {"impl_1315"}
// Dependencies: {}
impl < 'repo > Drop for TreeBuilder < 'repo > { fn drop (& mut self) { unsafe { raw :: git_treebuilder_free (self . raw) } } }
};
}
