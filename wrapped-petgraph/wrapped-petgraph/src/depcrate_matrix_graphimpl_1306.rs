// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_matrix_graphimpl_1306 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1306"}
// Dependencies: {}
impl < Ix : IndexType , S : BuildHasher > Iterator for NodeIdentifiers < '_ , Ix , S > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (NodeIndex :: new) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
