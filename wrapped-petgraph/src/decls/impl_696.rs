macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        EdgeType!();
        Edges!();
    };
}

macro_rules! impl_696 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoEdges for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
    };
}

impl_696!();