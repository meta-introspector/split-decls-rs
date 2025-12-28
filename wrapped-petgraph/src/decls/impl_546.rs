macro_rules! deps {
    () => {
        Csr!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > NodeCount for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_count (& self) -> usize { (* self) . node_count () } }
    };
}

impl_546!()