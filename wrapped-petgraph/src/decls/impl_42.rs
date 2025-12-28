macro_rules! deps {
    () => {
        IntoWeightedEdge!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < Ix , E > IntoWeightedEdge < E > for & (Ix , Ix , E) where Ix : Copy , E : Clone , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { self . clone () } }
    };
}

impl_42!()