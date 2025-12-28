macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Display for CachePadded < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . value , f) } }
    };
}

impl_79!()