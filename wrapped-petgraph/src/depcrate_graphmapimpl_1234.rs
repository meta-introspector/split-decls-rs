// Generated macro for impl_1234 (impl)
macro_rules! Depcrate_graphmapimpl_1234 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1234"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , S > visit :: IntoNodeIdentifiers for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type NodeIdentifiers = NodeIdentifiers < 'a , N , E , Ty > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers { iter : self . nodes . iter () , ty : self . ty , edge_ty : PhantomData , } } }
};
}
