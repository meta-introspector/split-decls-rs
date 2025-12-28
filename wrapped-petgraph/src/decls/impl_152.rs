macro_rules! deps {
    () => {
        EdgeFilteredEdges!();
        EdgeFiltered!();
        EdgeRef!();
        Edges!();
        FilterEdge!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdges for & 'a EdgeFiltered < G , F > where G : IntoEdges , F : FilterEdge < G :: EdgeRef > , { type Edges = EdgeFilteredEdges < 'a , G , G :: Edges , F > ; fn edges (self , n : G :: NodeId) -> Self :: Edges { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edges (n) , f : & self . 1 , } } }
    };
}

impl_152!()