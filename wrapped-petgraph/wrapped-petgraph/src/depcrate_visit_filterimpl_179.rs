// Generated macro for impl_179 (impl)
macro_rules! Depcrate_visit_filterimpl_179 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , G , F > IntoEdgesDirected for & 'a EdgeFiltered < G , F > where G : IntoEdgesDirected , F : FilterEdge < G :: EdgeRef > , { type EdgesDirected = EdgeFilteredEdges < 'a , G , G :: EdgesDirected , F > ; fn edges_directed (self , n : G :: NodeId , dir : Direction) -> Self :: EdgesDirected { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edges_directed (n , dir) , f : & self . 1 , } } }
};
}
