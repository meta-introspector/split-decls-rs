macro_rules! deps {
    () => {
        EdgeIndex!();
        EdgeReference!();
        IndexType!();
        NodeIndex!();
        EdgeRef!();
    };
}

macro_rules! impl_752 {
    () => {
        deps!();
        impl < Ix , E > visit :: EdgeRef for EdgeReference < '_ , E , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; type Weight = E ; fn source (& self) -> Self :: NodeId { self . node [0] } fn target (& self) -> Self :: NodeId { self . node [1] } fn weight (& self) -> & E { self . weight } fn id (& self) -> Self :: EdgeId { self . index } }
    };
}

impl_752!();