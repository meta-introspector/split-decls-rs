macro_rules! deps {
    () => {
        NeighborsDirected!();
        Neighbors!();
        Reversed!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < G > IntoNeighbors for Reversed < G > where G : IntoNeighborsDirected , { type Neighbors = G :: NeighborsDirected ; fn neighbors (self , n : G :: NodeId) -> G :: NeighborsDirected { self . 0 . neighbors_directed (n , Incoming) } }
    };
}

impl_172!()