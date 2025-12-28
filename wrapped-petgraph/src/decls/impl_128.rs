macro_rules! deps {
    () => {
        NodeFiltered!();
        EdgeRef!();
        NodeFilteredEdgeReferences!();
        FilterNode!();
        EdgeReferences!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdgeReferences for & 'a NodeFiltered < G , F > where G : IntoEdgeReferences , F : FilterNode < G :: NodeId > , { type EdgeRef = G :: EdgeRef ; type EdgeReferences = NodeFilteredEdgeReferences < 'a , G , G :: EdgeReferences , F > ; fn edge_references (self) -> Self :: EdgeReferences { NodeFilteredEdgeReferences { graph : PhantomData , iter : self . 0 . edge_references () , f : & self . 1 , } } }
    };
}

impl_128!()