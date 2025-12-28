macro_rules! deps {
    () => {
        NodeTrait!();
        EdgeType!();
        GraphMap!();
    };
}

macro_rules! impl_930 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: NodeCount for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { fn node_count (& self) -> usize { (* self) . node_count () } }
    };
}

impl_930!();