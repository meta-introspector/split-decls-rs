macro_rules! deps {
    () => {
        IndexType!();
        NodeRef!();
        NodeIndex!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < Ix : IndexType > visit :: NodeRef for NodeIndex < Ix > { type NodeId = NodeIndex < Ix > ; type Weight = () ; fn id (& self) -> Self :: NodeId { * self } fn weight (& self) -> & Self :: Weight { & () } }
    };
}

impl_303!()