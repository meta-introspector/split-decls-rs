macro_rules! deps {
    () => {
        EdgeFiltered!();
        FilterEdge!();
        EdgeRef!();
        EdgeReferences!();
        EdgeFilteredEdges!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'a , G , F > IntoEdgeReferences for & 'a EdgeFiltered < G , F > where G : IntoEdgeReferences , F : FilterEdge < G :: EdgeRef > , { type EdgeRef = G :: EdgeRef ; type EdgeReferences = EdgeFilteredEdges < 'a , G , G :: EdgeReferences , F > ; fn edge_references (self) -> Self :: EdgeReferences { EdgeFilteredEdges { graph : PhantomData , iter : self . 0 . edge_references () , f : & self . 1 , } } }
    };
}

impl_151!();