macro_rules! deps {
    () => {
        StableGraph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_853 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeCount for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_count (& self) -> usize { self . node_count () } }
    };
}

impl_853!();