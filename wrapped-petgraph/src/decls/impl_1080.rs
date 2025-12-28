macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_1080 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix) where E : Default , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (s , t) = self ; (s , t , E :: default ()) } }
    };
}

impl_1080!();