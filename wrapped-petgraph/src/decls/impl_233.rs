macro_rules! deps {
    () => {
        IndexType!();
        StableGraph!();
        EdgeType!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > DataMap for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . edge_weight (id) } }
    };
}

impl_233!();