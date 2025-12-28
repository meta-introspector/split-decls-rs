macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        List!();
        Directed!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: GraphProp for List < E , Ix > { type EdgeType = crate :: Directed ; fn is_directed (& self) -> bool { true } }
    };
}

impl_315!();