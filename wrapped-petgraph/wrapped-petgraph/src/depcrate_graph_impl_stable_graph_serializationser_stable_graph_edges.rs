// Generated macro for ser_stable_graph_edges (function)
macro_rules! Depcrate_graph_impl_stable_graph_serializationser_stable_graph_edges {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"ser_stable_graph_edges"}
// Dependencies: {}
fn ser_stable_graph_edges < S , E , Ix > (edges : & & [Edge < Option < E > , Ix >] , serializer : S ,) -> Result < S :: Ok , S :: Error > where S : Serializer , E : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (edges . iter () . map (| edge | { edge . weight . as_ref () . map (| w | (edge . source () , edge . target () , w)) })) }
};
}
