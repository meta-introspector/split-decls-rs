macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_1081 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix , E) { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { self } }
    };
}

impl_1081!();