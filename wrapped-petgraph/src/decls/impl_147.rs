macro_rules! deps {
    () => {
        EdgeFiltered!();
        FilterEdge!();
        Neighbors!();
        EdgeRef!();
        EdgeFilteredNeighbors!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNeighbors for & 'a EdgeFiltered < G , F > where G : IntoEdges , F : FilterEdge < G :: EdgeRef > , { type Neighbors = EdgeFilteredNeighbors < 'a , G , F > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { EdgeFilteredNeighbors { iter : self . 0 . edges (n) , f : & self . 1 , } } }
    };
}

impl_147!();