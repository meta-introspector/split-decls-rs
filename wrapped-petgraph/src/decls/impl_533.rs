macro_rules! deps {
    () => {
        Csr!();
        Edges!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > IntoEdges for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
    };
}

impl_533!()