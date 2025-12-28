macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
    };
}

macro_rules! impl_927 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: GraphProp for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type EdgeType = Ty ; }
    };
}

impl_927!()