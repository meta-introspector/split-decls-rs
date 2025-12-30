// Generated macro for impl_996 (impl)
macro_rules! Depcrate_graph_implimpl_996 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_996"}
// Dependencies: {}
impl < Ix : IndexType > DoubleEndedIterator for NodeIndices < Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . r . next_back () . map (node_index) } }
};
}
