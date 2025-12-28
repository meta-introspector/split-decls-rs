macro_rules! deps {
    () => {
        EdgeType!();
        MatrixGraph!();
        IndexType!();
        NodeIndex!();
        Nullable!();
    };
}

macro_rules! impl_1040 {
    () => {
        deps!();
        impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > NodeIndexable for MatrixGraph < N , E , S , Ty , Null , Ix > { fn node_bound (& self) -> usize { self . nodes . upper_bound } fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
    };
}

impl_1040!();