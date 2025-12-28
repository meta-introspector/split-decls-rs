macro_rules! deps {
    () => {
        NodeFilteredEdges!();
        FilterNode!();
        Edges!();
        NodeFiltered!();
        Direction!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdges for & 'a NodeFiltered < G , F > where G : IntoEdges , F : FilterNode < G :: NodeId > , { type Edges = NodeFilteredEdges < 'a , G , G :: Edges , F > ; fn edges (self , a : G :: NodeId) -> Self :: Edges { NodeFilteredEdges { graph : PhantomData , include_source : self . 1 . include_node (a) , iter : self . 0 . edges (a) , f : & self . 1 , dir : Direction :: Outgoing , } } }
    };
}

impl_131!()