macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for Iter < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_67!()