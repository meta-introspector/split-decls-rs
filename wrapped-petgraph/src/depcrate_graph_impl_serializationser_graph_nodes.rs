// Generated macro for ser_graph_nodes (function)
macro_rules! Depcrate_graph_impl_serializationser_graph_nodes {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"ser_graph_nodes"}
// Dependencies: {}
fn ser_graph_nodes < S , N , Ix > (nodes : & & [Node < N , Ix >] , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , N : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (nodes . iter () . map (| node | & node . weight)) }
};
}
