macro_rules! deps {
    () => {
        EdgeType!();
        GraphMap!();
        NodeTrait!();
    };
}

macro_rules! impl_932 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: NodeCompactIndexable for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { }
    };
}

impl_932!()