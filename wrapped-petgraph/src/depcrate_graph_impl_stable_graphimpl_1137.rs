// Generated macro for impl_1137 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1137 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1137"}
// Dependencies: {}
impl < E , Ix > Iterator for Neighbors < '_ , E , Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < NodeIndex < Ix > > { match self . edges . get (self . next [0] . index ()) { None => { } Some (edge) => { debug_assert ! (edge . weight . is_some ()) ; self . next [0] = edge . next [0] ; return Some (edge . node [1]) ; } } while let Some (edge) = self . edges . get (self . next [1] . index ()) { debug_assert ! (edge . weight . is_some ()) ; self . next [1] = edge . next [1] ; if edge . node [0] != self . skip_start { return Some (edge . node [0]) ; } } None } }
};
}
