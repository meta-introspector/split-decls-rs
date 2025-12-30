// Generated macro for impl_171 (impl)
macro_rules! Depcrate_graph_linked_graphimpl_171 {
() => {
// Module: crate::graph::linked_graph
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'g , N : Debug , E : Debug > AdjacentEdges < 'g , N , E > { fn targets (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . target) } fn sources (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . source) } }
};
}
