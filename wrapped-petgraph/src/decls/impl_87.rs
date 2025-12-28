macro_rules! deps {
    () => {
        NodeRef!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < Id > NodeRef for (Id , ()) where Id : Copy , { type NodeId = Id ; type Weight = () ; fn id (& self) -> Self :: NodeId { self . 0 } fn weight (& self) -> & Self :: Weight { static DUMMY : () = () ; & DUMMY } }
    };
}

impl_87!()