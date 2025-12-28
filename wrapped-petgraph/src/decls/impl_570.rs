macro_rules! deps {
    () => {
        Dot!();
    };
}

macro_rules! impl_570 {
    () => {
        deps!();
        impl < G > fmt :: Debug for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: Debug , G :: NodeWeight : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: Debug :: fmt , fmt :: Debug :: fmt) } }
    };
}

impl_570!()