macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
    };
}

macro_rules! impl_850 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: Data for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeWeight = N ; type EdgeWeight = E ; }
    };
}

impl_850!();