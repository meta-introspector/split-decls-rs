macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for CachePadded < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CachePadded") . field ("value" , & self . value) . finish () } }
    };
}

impl_77!();