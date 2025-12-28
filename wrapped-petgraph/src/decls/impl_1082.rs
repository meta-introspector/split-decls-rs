macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_1082 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix , & E) where E : Clone , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (a , b , c) = self ; (a , b , c . clone ()) } }
    };
}

impl_1082!()