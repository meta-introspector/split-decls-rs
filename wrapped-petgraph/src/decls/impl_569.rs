macro_rules! deps {
    () => {
        Dot!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        impl < G > fmt :: UpperHex for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: UpperHex , G :: NodeWeight : fmt :: UpperHex , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: UpperHex :: fmt , fmt :: UpperHex :: fmt) } }
    };
}

impl_569!();