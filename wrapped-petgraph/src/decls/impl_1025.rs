macro_rules! deps {
    () => {
        IndexType!();
        MatrixGraph!();
        Nullable!();
        EdgeType!();
    };
}

macro_rules! impl_1025 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > EdgeCount for MatrixGraph < N , E , S , Ty , Null , Ix > { # [inline] fn edge_count (& self) -> usize { self . edge_count () } }
    };
}

impl_1025!()