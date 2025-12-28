macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        EdgeType!();
    };
}

macro_rules! impl_737 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: GraphProp for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeType = Ty ; }
    };
}

impl_737!();