macro_rules! deps {
    () => {
        NeighborsDirected!();
        Direction!();
        GraphMap!();
        EdgeType!();
    };
}

macro_rules! impl_934 {
    () => {
        deps!();
        impl < 'a , N : 'a , E , Ty , S > visit :: IntoNeighborsDirected for & 'a GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type NeighborsDirected = NeighborsDirected < 'a , N , Ty > ; fn neighbors_directed (self , n : N , dir : Direction) -> Self :: NeighborsDirected { self . neighbors_directed (n , dir) } }
    };
}

impl_934!()