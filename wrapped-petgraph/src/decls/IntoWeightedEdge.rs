macro_rules! IntoWeightedEdge {
    () => {
        # [doc = " Convert an element like `(i, j)` or `(i, j, w)` into"] # [doc = " a triple of source, target, edge weight."] # [doc = ""] # [doc = " For `Graph::from_edges` and `GraphMap::from_edges`."] pub trait IntoWeightedEdge < E > { type NodeId ; fn into_weighted_edge (self) -> (Self :: NodeId , Self :: NodeId , E) ; }
    };
}

IntoWeightedEdge!()