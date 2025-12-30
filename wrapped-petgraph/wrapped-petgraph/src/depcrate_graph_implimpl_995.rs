// Generated macro for impl_995 (impl)
macro_rules! Depcrate_graph_implimpl_995 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_995"}
// Dependencies: {}
impl < Ix : IndexType > Iterator for NodeIndices < Ix > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (node_index) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
};
}
