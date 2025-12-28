macro_rules! deps {
    () => {
        Direction!();
        UndirectedAdaptor!();
        NeighborsDirected!();
        Neighbors!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < G > IntoNeighbors for UndirectedAdaptor < G > where G : IntoNeighborsDirected , { type Neighbors = core :: iter :: Chain < G :: NeighborsDirected , G :: NeighborsDirected > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { self . 0 . neighbors_directed (n , Direction :: Incoming) . chain (self . 0 . neighbors_directed (n , Direction :: Outgoing)) } }
    };
}

impl_198!();