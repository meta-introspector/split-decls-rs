macro_rules! deps {
    () => {
        EdgeFilteredNeighborsDirected!();
        Direction!();
        NeighborsDirected!();
        EdgeFiltered!();
        FilterEdge!();
        EdgeRef!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNeighborsDirected for & 'a EdgeFiltered < G , F > where G : IntoEdgesDirected , F : FilterEdge < G :: EdgeRef > , { type NeighborsDirected = EdgeFilteredNeighborsDirected < 'a , G , F > ; fn neighbors_directed (self , n : G :: NodeId , dir : Direction) -> Self :: NeighborsDirected { EdgeFilteredNeighborsDirected { iter : self . 0 . edges_directed (n , dir) , f : & self . 1 , from : n , } } }
    };
}

impl_148!();