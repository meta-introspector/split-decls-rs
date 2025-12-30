// Generated macro for impl_1142 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1142 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1142"}
// Dependencies: {}
impl < N , Ix : IndexType > Iterator for NodeIndices < '_ , N , Ix > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , node) | { if node . weight . is_some () { Some (node_index (i)) } else { None } }) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}
