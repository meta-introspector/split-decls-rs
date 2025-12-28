macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
    };
}

macro_rules! impl_939 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: EdgeCount for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { # [inline] fn edge_count (& self) -> usize { self . edge_count () } }
    };
}

impl_939!()