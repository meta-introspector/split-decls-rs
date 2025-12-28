macro_rules! deps {
    () => {
        Neighbors!();
        FilterNode!();
        NodeFiltered!();
        NodeFilteredNeighbors!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNeighbors for & 'a NodeFiltered < G , F > where G : IntoNeighbors , F : FilterNode < G :: NodeId > , { type Neighbors = NodeFilteredNeighbors < 'a , G :: Neighbors , F > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { NodeFilteredNeighbors { include_source : self . 1 . include_node (n) , iter : self . 0 . neighbors (n) , f : & self . 1 , } } }
    };
}

impl_120!()