macro_rules! deps {
    () => {
        EdgeType!();
        Acyclic!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < G : Visitable + GraphProp > GraphProp for Acyclic < G > { type EdgeType = G :: EdgeType ; }
    };
}

impl_270!()