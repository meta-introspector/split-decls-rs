macro_rules! deps {
    () => {
        NodeFiltered!();
        FilterNode!();
        NodeFilteredEdges!();
        EdgesDirected!();
        Direction!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdgesDirected for & 'a NodeFiltered < G , F > where G : IntoEdgesDirected , F : FilterNode < G :: NodeId > , { type EdgesDirected = NodeFilteredEdges < 'a , G , G :: EdgesDirected , F > ; fn edges_directed (self , a : G :: NodeId , dir : Direction) -> Self :: EdgesDirected { NodeFilteredEdges { graph : PhantomData , include_source : self . 1 . include_node (a) , iter : self . 0 . edges_directed (a , dir) , f : & self . 1 , dir , } } }
    };
}

impl_132!();