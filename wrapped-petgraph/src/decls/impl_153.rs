macro_rules! deps {
    () => {
        EdgesDirected!();
        EdgeFiltered!();
        EdgeRef!();
        Direction!();
        EdgeFilteredEdges!();
        FilterEdge!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdgesDirected for & 'a EdgeFiltered < G , F > where G : IntoEdgesDirected , F : FilterEdge < G :: EdgeRef > , { type EdgesDirected = EdgeFilteredEdges < 'a , G , G :: EdgesDirected , F > ; fn edges_directed (self , n : G :: NodeId , dir : Direction) -> Self :: EdgesDirected { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edges_directed (n , dir) , f : & self . 1 , } } }
    };
}

impl_153!();