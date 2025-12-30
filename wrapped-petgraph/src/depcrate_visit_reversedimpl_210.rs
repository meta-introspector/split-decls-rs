// Generated macro for impl_210 (impl)
macro_rules! Depcrate_visit_reversedimpl_210 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_210"}
// Dependencies: {}
impl < G > IntoEdgeReferences for Reversed < G > where G : IntoEdgeReferences , { type EdgeRef = ReversedEdgeReference < G :: EdgeRef > ; type EdgeReferences = ReversedEdgeReferences < G :: EdgeReferences > ; fn edge_references (self) -> Self :: EdgeReferences { ReversedEdgeReferences { iter : self . 0 . edge_references () , } } }
};
}
