// Generated macro for impl_234 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_234 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_234"}
// Dependencies: {}
impl < R > EdgeRef for MaybeReversedEdgeReference < R > where R : EdgeRef , { type NodeId = R :: NodeId ; type EdgeId = R :: EdgeId ; type Weight = R :: Weight ; fn source (& self) -> Self :: NodeId { if self . reversed { self . inner . target () } else { self . inner . source () } } fn target (& self) -> Self :: NodeId { if self . reversed { self . inner . source () } else { self . inner . target () } } fn weight (& self) -> & Self :: Weight { self . inner . weight () } fn id (& self) -> Self :: EdgeId { self . inner . id () } }
};
}
