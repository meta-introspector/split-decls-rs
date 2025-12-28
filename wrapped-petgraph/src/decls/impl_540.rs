macro_rules! deps {
    () => {
        EdgeType!();
        Csr!();
        IndexType!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > NodeCompactIndexable for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { }
    };
}

impl_540!()