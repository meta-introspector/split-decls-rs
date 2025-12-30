// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1128 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1128"}
// Dependencies: {}
impl < 'a , E , Ty , Ix > Iterator for EdgesConnecting < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < EdgeReference < 'a , E , Ix > > { let target_node = self . target_node ; self . edges . by_ref () . find (| & edge | edge . node [1] == target_node) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . edges . size_hint () ; (0 , upper) } }
};
}
