// Generated macro for ser_graph_edges (function)
macro_rules! Depcrate_graph_impl_serializationser_graph_edges {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"ser_graph_edges"}
// Dependencies: {}
fn ser_graph_edges < S , E , Ix > (edges : & & [Edge < E , Ix >] , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , E : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (edges . iter () . map (| edge | Some ((edge . source () , edge . target () , & edge . weight))) ,) }
};
}
