// Generated macro for impl_209 (impl)
macro_rules! Depcrate_visit_reversedimpl_209 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_209"}
// Dependencies: {}
# [doc = " An edge reference"] impl < R > EdgeRef for ReversedEdgeReference < R > where R : EdgeRef , { type NodeId = R :: NodeId ; type EdgeId = R :: EdgeId ; type Weight = R :: Weight ; fn source (& self) -> Self :: NodeId { self . 0 . target () } fn target (& self) -> Self :: NodeId { self . 0 . source () } fn weight (& self) -> & Self :: Weight { self . 0 . weight () } fn id (& self) -> Self :: EdgeId { self . 0 . id () } }
};
}
