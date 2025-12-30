// Generated macro for impl_1243 (impl)
macro_rules! Depcrate_graphmapimpl_1243 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1243"}
// Dependencies: {}
impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdgeReferences for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type EdgeRef = (N , N , & 'a E) ; type EdgeReferences = AllEdges < 'a , N , E , Ty > ; fn edge_references (self) -> Self :: EdgeReferences { self . all_edges () } }
};
}
