// Generated macro for impl_340 (impl)
macro_rules! Depcrate_adjimpl_340 {
() => {
// Module: crate::adj
// Provides: {"impl_340"}
// Dependencies: {}
impl < E , Ix : IndexType > visit :: EdgeRef for EdgeReference < '_ , E , Ix > { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . id . from } fn target (& self) -> Self :: NodeId { self . edge . suc } fn id (& self) -> Self :: EdgeId { self . id } fn weight (& self) -> & Self :: Weight { & self . edge . weight } }
};
}
