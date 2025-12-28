macro_rules! deps {
    () => {
        Graph!();
        IndexType!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Data for Graph < N , E , Ty , Ix > where Ix : IndexType , { type NodeWeight = N ; type EdgeWeight = E ; }
    };
}

impl_230!();