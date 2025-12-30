// Generated macro for impl_1093 (impl)
macro_rules! Depcrate_graph_impl_stable_graph_serializationimpl_1093 {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"impl_1093"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > IntoSerializable for & 'a StableGraph < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type Output = SerStableGraph < 'a , N , E , Ix > ; fn into_serializable (self) -> Self :: Output { let nodes = & self . raw_nodes () [.. self . node_bound ()] ; let node_count = self . node_count () ; let hole_count = nodes . len () - node_count ; let edges = & self . raw_edges () [.. self . edge_bound ()] ; SerStableGraph { nodes : Somes (node_count , nodes) , node_holes : Holes (hole_count , nodes) , edges , edge_property : EdgeProperty :: from (PhantomData :: < Ty >) , } } }
};
}
