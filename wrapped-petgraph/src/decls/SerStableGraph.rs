macro_rules! deps {
    () => {
        Holes!();
        Edge!();
        EdgeProperty!();
        Graph!();
        IndexType!();
        Somes!();
        Node!();
    };
}

macro_rules! SerStableGraph {
    () => {
        deps!();
        # [derive (Serialize)] # [serde (rename = "Graph")] # [serde (bound (serialize = "N: Serialize, E: Serialize, Ix: IndexType + Serialize"))] pub struct SerStableGraph < 'a , N : 'a , E : 'a , Ix : 'a + IndexType > { nodes : Somes < & 'a [Node < Option < N > , Ix >] > , node_holes : Holes < & 'a [Node < Option < N > , Ix >] > , edge_property : EdgeProperty , # [serde (serialize_with = "ser_stable_graph_edges")] edges : & 'a [Edge < Option < E > , Ix >] , }
    };
}

SerStableGraph!()