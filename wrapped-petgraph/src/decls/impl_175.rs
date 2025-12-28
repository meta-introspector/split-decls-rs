macro_rules! deps {
    () => {
        Reversed!();
        Edges!();
        Direction!();
        EdgesDirected!();
        ReversedEdges!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < G > IntoEdgesDirected for Reversed < G > where G : IntoEdgesDirected , { type EdgesDirected = ReversedEdges < G :: EdgesDirected > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: Edges { ReversedEdges { iter : self . 0 . edges_directed (a , dir . opposite ()) , } } }
    };
}

impl_175!();