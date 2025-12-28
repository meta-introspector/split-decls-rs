macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_1083 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for & (Ix , Ix) where Ix : Copy , E : Default , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (s , t) = * self ; (s , t , E :: default ()) } }
    };
}

impl_1083!()