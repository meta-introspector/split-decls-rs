macro_rules! deps {
    () => {
        ReversedEdgeReference!();
        Reversed!();
        EdgeReferences!();
        EdgeRef!();
        ReversedEdgeReferences!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < G > IntoEdgeReferences for Reversed < G > where G : IntoEdgeReferences , { type EdgeRef = ReversedEdgeReference < G :: EdgeRef > ; type EdgeReferences = ReversedEdgeReferences < G :: EdgeReferences > ; fn edge_references (self) -> Self :: EdgeReferences { ReversedEdgeReferences { iter : self . 0 . edge_references () , } } }
    };
}

impl_182!()