macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
    };
}

macro_rules! impl_925 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: Data for GraphMap < N , E , Ty , S > where N : Copy + PartialEq , Ty : EdgeType , S : BuildHasher , { type NodeWeight = N ; type EdgeWeight = E ; }
    };
}

impl_925!()