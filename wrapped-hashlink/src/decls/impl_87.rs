macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V > fmt :: Debug for Keys < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_87!();