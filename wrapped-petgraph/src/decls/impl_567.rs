macro_rules! deps {
    () => {
        Dot!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        impl < G > fmt :: Display for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: Display , G :: NodeWeight : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: Display :: fmt , fmt :: Display :: fmt) } }
    };
}

impl_567!();