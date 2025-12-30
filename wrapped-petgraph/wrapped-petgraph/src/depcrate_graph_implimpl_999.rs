// Generated macro for impl_999 (impl)
macro_rules! Depcrate_graph_implimpl_999 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_999"}
// Dependencies: {}
impl < Ix : IndexType > Iterator for EdgeIndices < Ix > { type Item = EdgeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (edge_index) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
};
}
