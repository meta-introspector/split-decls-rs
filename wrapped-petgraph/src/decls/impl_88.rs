macro_rules! deps {
    () => {
        NodeRef!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < Id , W > NodeRef for (Id , & W) where Id : Copy , { type NodeId = Id ; type Weight = W ; fn id (& self) -> Self :: NodeId { self . 0 } fn weight (& self) -> & Self :: Weight { self . 1 } }
    };
}

impl_88!();