macro_rules! deps {
    () => {
        Nullable!();
        Neighbors!();
        NodeIndex!();
        IndexType!();
        MatrixGraph!();
        EdgeType!();
    };
}

macro_rules! impl_1034 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNeighbors for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type Neighbors = Neighbors < 'a , Ty , Null , Ix > ; fn neighbors (self , a : NodeIndex < Ix >) -> Self :: Neighbors { MatrixGraph :: neighbors (self , a) } }
    };
}

impl_1034!();