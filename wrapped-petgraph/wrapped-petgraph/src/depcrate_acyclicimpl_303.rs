// Generated macro for impl_303 (impl)
macro_rules! Depcrate_acyclicimpl_303 {
() => {
// Module: crate::acyclic
// Provides: {"impl_303"}
// Dependencies: {}
impl < G : Build + Visitable + NodeIndexable > Build for Acyclic < G > where for < 'a > & 'a G : IntoNeighborsDirected + IntoNodeIdentifiers + Visitable < Map = G :: Map > + GraphBase < NodeId = G :: NodeId > , G :: NodeId : IndexType , { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId { let n = self . graph . add_node (weight) ; self . order_map . add_node (n , & self . graph) ; n } fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { self . try_add_edge (a , b , weight) . ok () } fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId { self . try_update_edge (a , b , weight) . unwrap () } }
};
}
