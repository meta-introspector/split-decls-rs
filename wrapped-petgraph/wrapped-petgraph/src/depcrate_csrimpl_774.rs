// Generated macro for impl_774 (impl)
macro_rules! Depcrate_csrimpl_774 {
() => {
// Module: crate::csr
// Provides: {"impl_774"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > IntoNodeReferences for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . node_weights . iter () . enumerate () , ty : PhantomData , } } }
};
}
