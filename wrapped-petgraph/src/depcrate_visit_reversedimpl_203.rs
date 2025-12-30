// Generated macro for impl_203 (impl)
macro_rules! Depcrate_visit_reversedimpl_203 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_203"}
// Dependencies: {}
impl < G > IntoEdgesDirected for Reversed < G > where G : IntoEdgesDirected , { type EdgesDirected = ReversedEdges < G :: EdgesDirected > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: Edges { ReversedEdges { iter : self . 0 . edges_directed (a , dir . opposite ()) , } } }
};
}
