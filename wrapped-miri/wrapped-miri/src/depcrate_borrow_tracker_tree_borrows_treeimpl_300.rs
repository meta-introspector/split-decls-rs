// Generated macro for impl_300 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeimpl_300 {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"impl_300"}
// Dependencies: {}
impl VisitProvenance for Tree { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { visit (None , Some (self . nodes . get (self . root) . unwrap () . tag)) ; for (_id , node) in self . nodes . iter () { if node . is_exposed { visit (None , Some (node . tag)) } } } }
};
}
