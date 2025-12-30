// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1158 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1158"}
// Dependencies: {}
impl < Ix , E > visit :: EdgeRef for EdgeReference < '_ , E , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . node [0] } fn target (& self) -> Self :: NodeId { self . node [1] } fn weight (& self) -> & E { self . weight } fn id (& self) -> Self :: EdgeId { self . index } }
};
}
