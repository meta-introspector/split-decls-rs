macro_rules! deps {
    () => {
        Csr!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > NodeIndexable for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_bound (& self) -> usize { self . node_count () } fn to_index (& self , a : Self :: NodeId) -> usize { a . index () } fn from_index (& self , ix : usize) -> Self :: NodeId { Ix :: new (ix) } }
    };
}

impl_539!();