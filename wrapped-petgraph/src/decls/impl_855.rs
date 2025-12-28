macro_rules! deps {
    () => {
        StableGraph!();
        NodeIndex!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_855 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeIndexable for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [doc = " Return an upper bound of the node indices in the graph"] fn node_bound (& self) -> usize { self . node_indices () . next_back () . map_or (0 , | i | i . index () + 1) } fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
    };
}

impl_855!();