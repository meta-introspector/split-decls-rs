// Generated macro for impl_286 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeimpl_286 {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"impl_286"}
// Dependencies: {}
impl AccessRelatedness { # [doc = " Check that access is either Ancestor or Distant, i.e. not"] # [doc = " a transitive child (initial pointer included)."] pub fn is_foreign (self) -> bool { matches ! (self , AccessRelatedness :: AncestorAccess | AccessRelatedness :: CousinAccess) } }
};
}
