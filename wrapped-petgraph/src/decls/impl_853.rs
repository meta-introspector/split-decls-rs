macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
    };
}

macro_rules! impl_853 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeCount for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_count (& self) -> usize { self . node_count () } }
    };
}

impl_853!()