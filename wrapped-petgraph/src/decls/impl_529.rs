macro_rules! deps {
    () => {
        IndexType!();
        Csr!();
        EdgeType!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Data for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeWeight = N ; type EdgeWeight = E ; }
    };
}

impl_529!();