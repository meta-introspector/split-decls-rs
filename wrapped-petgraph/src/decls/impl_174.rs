macro_rules! deps {
    () => {
        Reversed!();
        ReversedEdges!();
        EdgesDirected!();
        Edges!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < G > IntoEdges for Reversed < G > where G : IntoEdgesDirected , { type Edges = ReversedEdges < G :: EdgesDirected > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { ReversedEdges { iter : self . 0 . edges_directed (a , Incoming) , } } }
    };
}

impl_174!()