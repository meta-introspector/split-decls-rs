macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_739 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeCount for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_count (& self) -> usize { self . node_count () } }
    };
}

impl_739!()