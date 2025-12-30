// Generated macro for impl_1000 (impl)
macro_rules! Depcrate_graph_implimpl_1000 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1000"}
// Dependencies: {}
impl < Ix : IndexType > DoubleEndedIterator for EdgeIndices < Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . r . next_back () . map (edge_index) } }
};
}
