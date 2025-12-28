macro_rules! deps {
    () => {
        IndexType!();
        Node!();
        Edge!();
        Graph!();
        EdgeProperty!();
        StableGraph!();
        NodeIndex!();
    };
}

macro_rules! SerGraph {
    () => {
        deps!();
        # [doc = " Serialization representation for Graph"] # [doc = " Keep in sync with deserialization and StableGraph"] # [doc = ""] # [doc = " The serialization format is as follows, in Pseudorust:"] # [doc = ""] # [doc = " Graph {"] # [doc = "     nodes: [N],"] # [doc = "     node_holes: [NodeIndex<Ix>],"] # [doc = "     edge_property: EdgeProperty,"] # [doc = "     edges: [Option<(NodeIndex<Ix>, NodeIndex<Ix>, E)>]"] # [doc = " }"] # [doc = ""] # [doc = " The same format is used by both Graph and StableGraph."] # [doc = ""] # [doc = " For graph there are restrictions:"] # [doc = " node_holes is always empty and edges are always Some"] # [doc = ""] # [doc = " A stable graph serialization that obeys these restrictions"] # [doc = " (effectively, it has no interior vacancies) can de deserialized"] # [doc = " as a graph."] # [doc = ""] # [doc = " Node indices are serialized as integers and are fixed size for"] # [doc = " binary formats, so the Ix parameter matters there."] # [derive (Serialize)] # [serde (rename = "Graph")] # [serde (bound (serialize = "N: Serialize, E: Serialize, Ix: IndexType + Serialize"))] pub struct SerGraph < 'a , N : 'a , E : 'a , Ix : 'a + IndexType > { # [serde (serialize_with = "ser_graph_nodes")] nodes : & 'a [Node < N , Ix >] , node_holes : & 'a [NodeIndex < Ix >] , edge_property : EdgeProperty , # [serde (serialize_with = "ser_graph_edges")] edges : & 'a [Edge < E , Ix >] , }
    };
}

SerGraph!()