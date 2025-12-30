// Generated macro for impl_1233 (impl)
macro_rules! Depcrate_graphmapimpl_1233 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1233"}
// Dependencies: {}
impl < 'a , N , E , Ty , S > visit :: IntoNodeReferences for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type NodeRef = (N , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , E , Ty > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . nodes . iter () , ty : self . ty , edge_ty : PhantomData , } } }
};
}
