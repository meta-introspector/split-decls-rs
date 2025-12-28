macro_rules! deps {
    () => {
        IndexType!();
        EdgeType!();
        StableGraph!();
    };
}

macro_rules! impl_851 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: GraphProp for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeType = Ty ; }
    };
}

impl_851!();