// Generated macro for impl_284 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeimpl_284 {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"impl_284"}
// Dependencies: {}
impl VisitProvenance for Tree { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { visit (None , Some (self . nodes . get (self . root) . unwrap () . tag)) } }
};
}
