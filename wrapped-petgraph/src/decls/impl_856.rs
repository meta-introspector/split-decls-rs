macro_rules! deps {
    () => {
        StableGraph!();
        EdgeType!();
        IndexType!();
        Neighbors!();
    };
}

macro_rules! impl_856 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighbors for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Neighbors = Neighbors < 'a , E , Ix > ; fn neighbors (self , n : Self :: NodeId) -> Self :: Neighbors { (* self) . neighbors (n) } }
    };
}

impl_856!();