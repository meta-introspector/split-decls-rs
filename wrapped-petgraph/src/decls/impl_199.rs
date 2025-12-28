macro_rules! deps {
    () => {
        UndirectedAdaptor!();
        MaybeReversedEdges!();
        EdgesDirected!();
        Direction!();
        Edges!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < G > IntoEdges for UndirectedAdaptor < G > where G : IntoEdgesDirected , { type Edges = core :: iter :: Chain < MaybeReversedEdges < G :: EdgesDirected > , MaybeReversedEdges < G :: EdgesDirected > , > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { let incoming = MaybeReversedEdges { iter : self . 0 . edges_directed (a , Direction :: Incoming) , reversed : true , } ; let outgoing = MaybeReversedEdges { iter : self . 0 . edges_directed (a , Direction :: Outgoing) , reversed : false , } ; incoming . chain (outgoing) } }
    };
}

impl_199!()