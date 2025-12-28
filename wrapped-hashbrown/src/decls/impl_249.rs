macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < K : Debug , V : Debug > fmt :: Debug for Iter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_249!()