// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1145 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1145"}
// Dependencies: {}
impl < E , Ix : IndexType > Iterator for EdgeIndices < '_ , E , Ix > { type Item = EdgeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , node) | { if node . weight . is_some () { Some (edge_index (i)) } else { None } }) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}
