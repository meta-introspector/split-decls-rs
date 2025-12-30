// Generated macro for impl_911 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_911 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_911"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > IntoSerializable for & 'a Graph < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type Output = SerGraph < 'a , N , E , Ix > ; fn into_serializable (self) -> Self :: Output { SerGraph { nodes : & self . nodes , node_holes : & [] , edges : & self . edges , edge_property : EdgeProperty :: from (PhantomData :: < Ty >) , } } }
};
}
