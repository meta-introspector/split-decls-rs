macro_rules! deps {
    () => {
        Graph!();
        NodeIndex!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_740 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeIndexable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [inline] fn node_bound (& self) -> usize { self . node_count () } # [inline] fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } # [inline] fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
    };
}

impl_740!()