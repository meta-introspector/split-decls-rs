// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1143 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1143"}
// Dependencies: {}
impl < N , Ix : IndexType > DoubleEndedIterator for NodeIndices < '_ , N , Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | { if node . weight . is_some () { Some (node_index (i)) } else { None } }) } }
};
}
