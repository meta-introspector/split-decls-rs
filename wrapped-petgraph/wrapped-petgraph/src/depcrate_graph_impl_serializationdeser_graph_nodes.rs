// Generated macro for deser_graph_nodes (function)
macro_rules! Depcrate_graph_impl_serializationdeser_graph_nodes {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"deser_graph_nodes"}
// Dependencies: {}
fn deser_graph_nodes < 'de , D , N , Ix > (deserializer : D) -> Result < Vec < Node < N , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: new (| n | { Ok (Node { weight : n , next : [EdgeIndex :: end () ; 2] , }) })) }
};
}
