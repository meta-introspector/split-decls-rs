macro_rules! deps {
    () => {
        GraphMap!();
    };
}

macro_rules! impl_924 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: GraphBase for GraphMap < N , E , Ty , S > where N : Copy + PartialEq , S : BuildHasher , { type NodeId = N ; type EdgeId = (N , N) ; }
    };
}

impl_924!()