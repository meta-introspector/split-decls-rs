macro_rules! deps {
    () => {
        Graph!();
        StableGraph!();
        NodeIndex!();
        Edge!();
        EdgeType!();
        IndexType!();
        Node!();
        EdgeIndex!();
    };
}

macro_rules! impl_816 {
    () => {
        deps!();
        # [doc = " Convert a `Graph` into a `StableGraph`"] # [doc = ""] # [doc = " Computes in **O(|V| + |E|)** time where V is the set of nodes and E is the set of edges."] # [doc = ""] # [doc = " The resulting graph has the same node and edge indices as"] # [doc = " the original graph."] impl < N , E , Ty , Ix > From < Graph < N , E , Ty , Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from (g : Graph < N , E , Ty , Ix >) -> Self { let nodes = g . nodes . into_iter () . map (| e | Node { weight : Some (e . weight) , next : e . next , }) ; let edges = g . edges . into_iter () . map (| e | Edge { weight : Some (e . weight) , node : e . node , next : e . next , }) ; StableGraph { node_count : nodes . len () , edge_count : edges . len () , g : Graph { edges : edges . collect () , nodes : nodes . collect () , ty : g . ty , } , free_node : NodeIndex :: end () , free_edge : EdgeIndex :: end () , } } }
    };
}

impl_816!();