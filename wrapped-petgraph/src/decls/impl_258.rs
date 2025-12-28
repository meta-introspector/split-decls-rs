macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < G : Visitable > GraphBase for Acyclic < G > { type NodeId = G :: NodeId ; type EdgeId = G :: EdgeId ; }
    };
}

impl_258!()