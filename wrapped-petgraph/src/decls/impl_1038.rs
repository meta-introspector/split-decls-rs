macro_rules! deps {
    () => {
        MatrixGraph!();
        IndexType!();
        Edges!();
        EdgeType!();
        Nullable!();
    };
}

macro_rules! impl_1038 {
    () => {
        deps!();
        impl < 'a , N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdges for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type Edges = Edges < 'a , Ty , Null , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { MatrixGraph :: edges (self , a) } }
    };
}

impl_1038!()