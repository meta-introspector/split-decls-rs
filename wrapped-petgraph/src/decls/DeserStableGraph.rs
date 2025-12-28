macro_rules! deps {
    () => {
        EdgeProperty!();
        Graph!();
        NodeIndex!();
        IndexType!();
        Node!();
        Edge!();
    };
}

macro_rules! DeserStableGraph {
    () => {
        deps!();
        # [derive (Deserialize)] # [serde (rename = "Graph")] # [serde (bound (deserialize = "N: Deserialize<'de>, E: Deserialize<'de>, Ix: IndexType + Deserialize<'de>"))] pub struct DeserStableGraph < N , E , Ix > { # [serde (deserialize_with = "deser_stable_graph_nodes")] nodes : Vec < Node < Option < N > , Ix > > , # [serde (default = "Vec::new")] node_holes : Vec < NodeIndex < Ix > > , edge_property : EdgeProperty , # [serde (deserialize_with = "deser_stable_graph_edges")] edges : Vec < Edge < Option < E > , Ix > > , }
    };
}

DeserStableGraph!();