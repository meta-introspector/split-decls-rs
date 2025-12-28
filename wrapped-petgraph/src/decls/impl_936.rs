macro_rules! deps {
    () => {
        EdgeType!();
        NodeTrait!();
        Edges!();
        GraphMap!();
    };
}

macro_rules! impl_936 {
    () => {
        deps!();
        impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdges for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type Edges = Edges < 'a , N , E , Ty , S > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
    };
}

impl_936!();