// Generated macro for DeserGraph (struct)
macro_rules! Depcrate_graph_impl_serializationDeserGraph {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"DeserGraph"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (rename = "Graph")] # [serde (bound (deserialize = "N: Deserialize<'de>, E: Deserialize<'de>, Ix: IndexType + Deserialize<'de>"))] pub struct DeserGraph < N , E , Ix > { # [serde (deserialize_with = "deser_graph_nodes")] nodes : Vec < Node < N , Ix > > , # [serde (deserialize_with = "deser_graph_node_holes")] # [allow (unused)] # [serde (default = "Vec::new")] node_holes : Vec < NodeIndex < Ix > > , edge_property : EdgeProperty , # [serde (deserialize_with = "deser_graph_edges")] edges : Vec < Edge < E , Ix > > , }
};
}
