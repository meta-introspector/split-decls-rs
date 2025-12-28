macro_rules! deps {
    () => {
        Graph!();
        EdgeIndex!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_757 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: EdgeIndexable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn edge_bound (& self) -> usize { self . edge_count () } fn to_index (& self , ix : EdgeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: EdgeId { EdgeIndex :: new (ix) } }
    };
}

impl_757!();