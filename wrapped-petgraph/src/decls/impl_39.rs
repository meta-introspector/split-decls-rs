macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix , E) { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { self } }
    };
}

impl_39!()