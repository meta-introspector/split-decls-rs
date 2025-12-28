macro_rules! deps {
    () => {
        Csr!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > EdgeCount for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [inline] fn edge_count (& self) -> usize { self . edge_count () } }
    };
}

impl_547!()