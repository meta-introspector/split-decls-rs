macro_rules! deps {
    () => {
        IndexType!();
        EdgeType!();
        StableGraph!();
        Edges!();
    };
}

macro_rules! impl_858 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoEdges for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
    };
}

impl_858!()