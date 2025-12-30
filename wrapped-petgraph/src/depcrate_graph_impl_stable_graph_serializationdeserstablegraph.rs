// Generated macro for DeserStableGraph (struct)
macro_rules! Depcrate_graph_impl_stable_graph_serializationDeserStableGraph {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"DeserStableGraph"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (rename = "Graph")] # [serde (bound (deserialize = "N: Deserialize<'de>, E: Deserialize<'de>, Ix: IndexType + Deserialize<'de>"))] pub struct DeserStableGraph < N , E , Ix > { # [serde (deserialize_with = "deser_stable_graph_nodes")] nodes : Vec < Node < Option < N > , Ix > > , # [serde (default = "Vec::new")] node_holes : Vec < NodeIndex < Ix > > , edge_property : EdgeProperty , # [serde (deserialize_with = "deser_stable_graph_edges")] edges : Vec < Edge < Option < E > , Ix > > , }
};
}
