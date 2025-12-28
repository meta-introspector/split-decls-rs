macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < K , V : fmt :: Debug > fmt :: Debug for Values < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_94!()