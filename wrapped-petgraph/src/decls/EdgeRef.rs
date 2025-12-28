macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! EdgeRef {
    () => {
        deps!();
        # [doc = " An edge reference."] # [doc = ""] # [doc = " Edge references are used by traits `IntoEdges` and `IntoEdgeReferences`."] pub trait EdgeRef : Copy { type NodeId ; type EdgeId ; type Weight ; # [doc = " The source node of the edge."] fn source (& self) -> Self :: NodeId ; # [doc = " The target node of the edge."] fn target (& self) -> Self :: NodeId ; # [doc = " A reference to the weight of the edge."] fn weight (& self) -> & Self :: Weight ; # [doc = " The edge’s identifier."] fn id (& self) -> Self :: EdgeId ; }
    };
}

EdgeRef!()