macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < G : Visitable + Data > Data for Acyclic < G > { type NodeWeight = G :: NodeWeight ; type EdgeWeight = G :: EdgeWeight ; }
    };
}

impl_264!();