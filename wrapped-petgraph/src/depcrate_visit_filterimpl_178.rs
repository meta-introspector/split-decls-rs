// Generated macro for impl_178 (impl)
macro_rules! Depcrate_visit_filterimpl_178 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , G , F > IntoEdges for & 'a EdgeFiltered < G , F > where G : IntoEdges , F : FilterEdge < G :: EdgeRef > , { type Edges = EdgeFilteredEdges < 'a , G , G :: Edges , F > ; fn edges (self , n : G :: NodeId) -> Self :: Edges { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edges (n) , f : & self . 1 , } } }
};
}
