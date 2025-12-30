// Generated macro for impl_756 (impl)
macro_rules! Depcrate_csrimpl_756 {
() => {
// Module: crate::csr
// Provides: {"impl_756"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > IntoEdges for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
};
}
