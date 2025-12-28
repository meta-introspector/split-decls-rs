macro_rules! deps {
    () => {
        NeighborsDirected!();
        FilterNode!();
        Direction!();
        NodeFiltered!();
        NodeFilteredNeighbors!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNeighborsDirected for & 'a NodeFiltered < G , F > where G : IntoNeighborsDirected , F : FilterNode < G :: NodeId > , { type NeighborsDirected = NodeFilteredNeighbors < 'a , G :: NeighborsDirected , F > ; fn neighbors_directed (self , n : G :: NodeId , dir : Direction) -> Self :: NeighborsDirected { NodeFilteredNeighbors { include_source : self . 1 . include_node (n) , iter : self . 0 . neighbors_directed (n , dir) , f : & self . 1 , } } }
    };
}

impl_123!();