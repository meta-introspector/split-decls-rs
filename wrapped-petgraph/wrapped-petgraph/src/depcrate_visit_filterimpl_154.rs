// Generated macro for impl_154 (impl)
macro_rules! Depcrate_visit_filterimpl_154 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a , G , F > IntoEdgeReferences for & 'a NodeFiltered < G , F > where G : IntoEdgeReferences , F : FilterNode < G :: NodeId > , { type EdgeRef = G :: EdgeRef ; type EdgeReferences = NodeFilteredEdgeReferences < 'a , G , G :: EdgeReferences , F > ; fn edge_references (self) -> Self :: EdgeReferences { NodeFilteredEdgeReferences { graph : PhantomData , iter : self . 0 . edge_references () , f : & self . 1 , } } }
};
}
