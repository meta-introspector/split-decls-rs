// Generated macro for impl_177 (impl)
macro_rules! Depcrate_visit_filterimpl_177 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a , G , F > IntoEdgeReferences for & 'a EdgeFiltered < G , F > where G : IntoEdgeReferences , F : FilterEdge < G :: EdgeRef > , { type EdgeRef = G :: EdgeRef ; type EdgeReferences = EdgeFilteredEdges < 'a , G , G :: EdgeReferences , F > ; fn edge_references (self) -> Self :: EdgeReferences { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edge_references () , f : & self . 1 , } } }
};
}
