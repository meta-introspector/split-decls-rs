macro_rules! deps {
    () => {
        EdgeIndex!();
        IndexType!();
        EdgeReference!();
        NodeIndex!();
        EdgeRef!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: EdgeRef for EdgeReference < '_ , E , Ix > { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . id . from } fn target (& self) -> Self :: NodeId { self . edge . suc } fn id (& self) -> Self :: EdgeId { self . id } fn weight (& self) -> & Self :: Weight { & self . edge . weight } }
    };
}

impl_290!()