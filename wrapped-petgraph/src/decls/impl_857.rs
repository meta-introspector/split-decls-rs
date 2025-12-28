macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
        NeighborsDirected!();
        Neighbors!();
        Direction!();
        StableGraph!();
        EdgeType!();
    };
}

macro_rules! impl_857 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighborsDirected for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NeighborsDirected = Neighbors < 'a , E , Ix > ; fn neighbors_directed (self , n : NodeIndex < Ix > , d : Direction) -> Self :: NeighborsDirected { StableGraph :: neighbors_directed (self , n , d) } }
    };
}

impl_857!()