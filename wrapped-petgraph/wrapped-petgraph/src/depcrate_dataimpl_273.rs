// Generated macro for impl_273 (impl)
macro_rules! Depcrate_dataimpl_273 {
() => {
// Module: crate::data
// Provides: {"impl_273"}
// Dependencies: {}
# [cfg (feature = "graphmap")] impl < N , E , Ty , S > Build for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait , S : BuildHasher , { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId { self . add_node (weight) } fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { if self . contains_edge (a , b) { None } else { let r = self . add_edge (a , b , weight) ; debug_assert ! (r . is_none ()) ; Some ((a , b)) } } fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId { self . add_edge (a , b , weight) ; (a , b) } }
};
}
