macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: NodeCompactIndexable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { }
    };
}

impl_741!();