macro_rules! deps {
    () => {
        Direction!();
        Nullable!();
        NodeIndex!();
        NeighborsDirected!();
        IndexType!();
        Neighbors!();
        MatrixGraph!();
        Directed!();
    };
}

macro_rules! impl_1035 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , S : BuildHasher , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNeighborsDirected for & 'a MatrixGraph < N , E , S , Directed , Null , Ix > { type NeighborsDirected = Neighbors < 'a , Directed , Null , Ix > ; fn neighbors_directed (self , a : NodeIndex < Ix > , d : Direction) -> Self :: NeighborsDirected { MatrixGraph :: neighbors_directed (self , a , d) } }
    };
}

impl_1035!()