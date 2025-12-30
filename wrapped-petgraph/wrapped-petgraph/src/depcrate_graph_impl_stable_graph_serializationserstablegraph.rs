// Generated macro for SerStableGraph (struct)
macro_rules! Depcrate_graph_impl_stable_graph_serializationSerStableGraph {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"SerStableGraph"}
// Dependencies: {}
# [derive (Serialize)] # [serde (rename = "Graph")] # [serde (bound (serialize = "N: Serialize, E: Serialize, Ix: IndexType + Serialize"))] pub struct SerStableGraph < 'a , N : 'a , E : 'a , Ix : 'a + IndexType > { nodes : Somes < & 'a [Node < Option < N > , Ix >] > , node_holes : Holes < & 'a [Node < Option < N > , Ix >] > , edge_property : EdgeProperty , # [serde (serialize_with = "ser_stable_graph_edges")] edges : & 'a [Edge < Option < E > , Ix >] , }
};
}
