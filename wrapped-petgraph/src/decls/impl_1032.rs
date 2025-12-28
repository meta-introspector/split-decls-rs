macro_rules! deps {
    () => {
        Nullable!();
        EdgeType!();
        MatrixGraph!();
        IndexType!();
    };
}

macro_rules! impl_1032 {
    () => {
        deps!();
        impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Data for MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeWeight = N ; type EdgeWeight = E ; }
    };
}

impl_1032!()