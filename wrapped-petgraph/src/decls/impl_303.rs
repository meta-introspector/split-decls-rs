macro_rules! deps {
    () => {
        NodeRef!();
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < Ix : IndexType > visit :: NodeRef for NodeIndex < Ix > { type NodeId = NodeIndex < Ix > ; type Weight = () ; fn id (& self) -> Self :: NodeId { * self } fn weight (& self) -> & Self :: Weight { & () } }
    };
}

impl_303!();