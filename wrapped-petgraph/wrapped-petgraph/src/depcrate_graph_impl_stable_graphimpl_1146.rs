// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1146 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1146"}
// Dependencies: {}
impl < E , Ix : IndexType > DoubleEndedIterator for EdgeIndices < '_ , E , Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | { if node . weight . is_some () { Some (edge_index (i)) } else { None } }) } }
};
}
