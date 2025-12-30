// Generated macro for impl_71 (impl)
macro_rules! Depcrate_visit_traversalimpl_71 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_71"}
// Dependencies: {}
impl < W , C > Iterator for WalkerIter < W , C > where W : Walker < C > , C : Clone , { type Item = W :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . walker . walk_next (self . context . clone ()) } }
};
}
