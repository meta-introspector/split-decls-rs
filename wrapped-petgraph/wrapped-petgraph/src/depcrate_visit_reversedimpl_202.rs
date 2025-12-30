// Generated macro for impl_202 (impl)
macro_rules! Depcrate_visit_reversedimpl_202 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_202"}
// Dependencies: {}
impl < G > IntoEdges for Reversed < G > where G : IntoEdgesDirected , { type Edges = ReversedEdges < G :: EdgesDirected > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { ReversedEdges { iter : self . 0 . edges_directed (a , Incoming) , } } }
};
}
