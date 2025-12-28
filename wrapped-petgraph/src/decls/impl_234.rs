macro_rules! deps {
    () => {
        StableGraph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > DataMapMut for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight_mut (& mut self , id : Self :: NodeId) -> Option < & mut Self :: NodeWeight > { self . node_weight_mut (id) } fn edge_weight_mut (& mut self , id : Self :: EdgeId) -> Option < & mut Self :: EdgeWeight > { self . edge_weight_mut (id) } }
    };
}

impl_234!()