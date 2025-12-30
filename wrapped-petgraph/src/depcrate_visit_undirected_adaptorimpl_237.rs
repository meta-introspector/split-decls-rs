// Generated macro for impl_237 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_237 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_237"}
// Dependencies: {}
impl < G > IntoEdgeReferences for UndirectedAdaptor < G > where G : IntoEdgeReferences , { type EdgeRef = MaybeReversedEdgeReference < G :: EdgeRef > ; type EdgeReferences = MaybeReversedEdgeReferences < G :: EdgeReferences > ; fn edge_references (self) -> Self :: EdgeReferences { MaybeReversedEdgeReferences { iter : self . 0 . edge_references () , } } }
};
}
