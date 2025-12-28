macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
        Build!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > Build for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId { self . add_node (weight) } fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { Some (self . add_edge (a , b , weight)) } fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId { self . update_edge (a , b , weight) } }
    };
}

impl_236!()