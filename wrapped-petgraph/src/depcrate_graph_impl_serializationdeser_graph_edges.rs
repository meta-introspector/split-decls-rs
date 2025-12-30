// Generated macro for deser_graph_edges (function)
macro_rules! Depcrate_graph_impl_serializationdeser_graph_edges {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"deser_graph_edges"}
// Dependencies: {}
fn deser_graph_edges < 'de , D , N , Ix > (deserializer : D) -> Result < Vec < Edge < N , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: < Option < (NodeIndex < Ix > , NodeIndex < Ix > , N) > , _ , _ , > :: new (| x | { if let Some ((i , j , w)) = x { Ok (Edge { weight : w , node : [i , j] , next : [EdgeIndex :: end () ; 2] , }) } else { Err ("Graph can not have holes in the edge set, found None, expected edge") } })) }
};
}
