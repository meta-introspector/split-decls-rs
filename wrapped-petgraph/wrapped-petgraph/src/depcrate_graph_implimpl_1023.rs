// Generated macro for impl_1023 (impl)
macro_rules! Depcrate_graph_implimpl_1023 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1023"}
// Dependencies: {}
impl < Ix , E > visit :: EdgeRef for EdgeReference < '_ , E , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . node [0] } fn target (& self) -> Self :: NodeId { self . node [1] } fn weight (& self) -> & E { self . weight } fn id (& self) -> Self :: EdgeId { self . index } }
};
}
