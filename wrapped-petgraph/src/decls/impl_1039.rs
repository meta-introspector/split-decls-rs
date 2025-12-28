macro_rules! deps {
    () => {
        Direction!();
        EdgesDirected!();
        Directed!();
        IndexType!();
        MatrixGraph!();
        Nullable!();
        Edges!();
    };
}

macro_rules! impl_1039 {
    () => {
        deps!();
        impl < 'a , N , E , S : BuildHasher , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdgesDirected for & 'a MatrixGraph < N , E , S , Directed , Null , Ix > { type EdgesDirected = Edges < 'a , Directed , Null , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { MatrixGraph :: edges_directed (self , a , dir) } }
    };
}

impl_1039!()