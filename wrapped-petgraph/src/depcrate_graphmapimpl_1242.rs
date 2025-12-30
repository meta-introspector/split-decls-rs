// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_graphmapimpl_1242 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1242"}
// Dependencies: {}
impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdgesDirected for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type EdgesDirected = EdgesDirected < 'a , N , E , Ty , S > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
};
}
