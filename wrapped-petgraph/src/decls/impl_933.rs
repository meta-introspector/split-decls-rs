macro_rules! deps {
    () => {
        Neighbors!();
        EdgeType!();
        GraphMap!();
    };
}

macro_rules! impl_933 {
    () => {
        deps!();
        impl < 'a , N : 'a , E , Ty , S > visit :: IntoNeighbors for & 'a GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type Neighbors = Neighbors < 'a , N , Ty > ; fn neighbors (self , n : Self :: NodeId) -> Self :: Neighbors { self . neighbors (n) } }
    };
}

impl_933!()