macro_rules! deps {
    () => {
        UndirectedAdaptor!();
        EdgeReferences!();
        EdgeRef!();
        MaybeReversedEdgeReferences!();
        MaybeReversedEdgeReference!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < G > IntoEdgeReferences for UndirectedAdaptor < G > where G : IntoEdgeReferences , { type EdgeRef = MaybeReversedEdgeReference < G :: EdgeRef > ; type EdgeReferences = MaybeReversedEdgeReferences < G :: EdgeReferences > ; fn edge_references (self) -> Self :: EdgeReferences { MaybeReversedEdgeReferences { iter : self . 0 . edge_references () , } } }
    };
}

impl_207!()