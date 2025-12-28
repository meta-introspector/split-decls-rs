macro_rules! deps {
    () => {
        EdgeType!();
        Nullable!();
        MatrixGraph!();
        IndexType!();
    };
}

macro_rules! impl_1024 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > NodeCount for MatrixGraph < N , E , S , Ty , Null , Ix > { fn node_count (& self) -> usize { MatrixGraph :: node_count (self) } }
    };
}

impl_1024!()