macro_rules! deps {
    () => {
        EdgeRef!();
        MaybeReversedEdgeReference!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < R > EdgeRef for MaybeReversedEdgeReference < R > where R : EdgeRef , { type NodeId = R :: NodeId ; type EdgeId = R :: EdgeId ; type Weight = R :: Weight ; fn source (& self) -> Self :: NodeId { if self . reversed { self . inner . target () } else { self . inner . source () } } fn target (& self) -> Self :: NodeId { if self . reversed { self . inner . source () } else { self . inner . target () } } fn weight (& self) -> & Self :: Weight { self . inner . weight () } fn id (& self) -> Self :: EdgeId { self . inner . id () } }
    };
}

impl_204!()