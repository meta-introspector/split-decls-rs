macro_rules! deps {
    () => {
        IndexType!();
        Csr!();
        EdgeType!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > GraphProp for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeType = Ty ; }
    };
}

impl_548!();