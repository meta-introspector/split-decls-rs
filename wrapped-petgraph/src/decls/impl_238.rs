macro_rules! deps {
    () => {
        Create!();
        Graph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Create for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn with_capacity (nodes : usize , edges : usize) -> Self { Self :: with_capacity (nodes , edges) } }
    };
}

impl_238!()