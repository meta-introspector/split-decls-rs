// Generated macro for impl_750 (impl)
macro_rules! Depcrate_csrimpl_750 {
() => {
// Module: crate::csr
// Provides: {"impl_750"}
// Dependencies: {}
impl < E , Ty , Ix > EdgeRef for EdgeReference < '_ , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . source } fn target (& self) -> Self :: NodeId { self . target } fn weight (& self) -> & E { self . weight } fn id (& self) -> Self :: EdgeId { self . index } }
};
}
