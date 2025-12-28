macro_rules! deps {
    () => {
        Nullable!();
        EdgeType!();
        MatrixGraph!();
        IndexType!();
    };
}

macro_rules! impl_1031 {
    () => {
        deps!();
        impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > GraphProp for MatrixGraph < N , E , S , Ty , Null , Ix > { type EdgeType = Ty ; }
    };
}

impl_1031!()