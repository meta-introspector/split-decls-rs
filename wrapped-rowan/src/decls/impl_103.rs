macro_rules! deps {
    () => {
        NodeOrToken!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < N : fmt :: Display , T : fmt :: Display > fmt :: Display for NodeOrToken < N , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { NodeOrToken :: Node (node) => fmt :: Display :: fmt (node , f) , NodeOrToken :: Token (token) => fmt :: Display :: fmt (token , f) , } } }
    };
}

impl_103!()