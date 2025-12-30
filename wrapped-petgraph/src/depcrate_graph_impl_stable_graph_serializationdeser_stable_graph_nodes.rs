// Generated macro for deser_stable_graph_nodes (function)
macro_rules! Depcrate_graph_impl_stable_graph_serializationdeser_stable_graph_nodes {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"deser_stable_graph_nodes"}
// Dependencies: {}
fn deser_stable_graph_nodes < 'de , D , N , Ix > (deserializer : D ,) -> Result < Vec < Node < Option < N > , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: new (| n | { Ok (Node { weight : Some (n) , next : [EdgeIndex :: end () ; 2] , }) })) }
};
}
