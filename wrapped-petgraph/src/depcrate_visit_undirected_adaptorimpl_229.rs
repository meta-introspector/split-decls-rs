// Generated macro for impl_229 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_229 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_229"}
// Dependencies: {}
impl < G > IntoEdges for UndirectedAdaptor < G > where G : IntoEdgesDirected , { type Edges = core :: iter :: Chain < MaybeReversedEdges < G :: EdgesDirected > , MaybeReversedEdges < G :: EdgesDirected > , > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { let incoming = MaybeReversedEdges { iter : self . 0 . edges_directed (a , Direction :: Incoming) , reversed : true , } ; let outgoing = MaybeReversedEdges { iter : self . 0 . edges_directed (a , Direction :: Outgoing) , reversed : false , } ; incoming . chain (outgoing) } }
};
}
