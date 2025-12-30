// Generated macro for impl_770 (impl)
macro_rules! Depcrate_csrimpl_770 {
() => {
// Module: crate::csr
// Provides: {"impl_770"}
// Dependencies: {}
impl < N , E , Ty , Ix > IntoNodeIdentifiers for & Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIdentifiers < Ix > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers { r : 0 .. self . node_count () , ty : PhantomData , } } }
};
}
