macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < K , V : Debug > fmt :: Debug for Values < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_272!()