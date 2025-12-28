macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        Graph!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > DataMap for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . edge_weight (id) } }
    };
}

impl_231!()