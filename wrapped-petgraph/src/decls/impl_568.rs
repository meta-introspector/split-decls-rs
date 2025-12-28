macro_rules! deps {
    () => {
        Dot!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < G > fmt :: LowerHex for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: LowerHex , G :: NodeWeight : fmt :: LowerHex , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: LowerHex :: fmt , fmt :: LowerHex :: fmt) } }
    };
}

impl_568!();