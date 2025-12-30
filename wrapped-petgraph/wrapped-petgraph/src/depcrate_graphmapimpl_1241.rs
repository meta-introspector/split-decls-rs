// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_graphmapimpl_1241 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1241"}
// Dependencies: {}
impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdges for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type Edges = Edges < 'a , N , E , Ty , S > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
};
}
