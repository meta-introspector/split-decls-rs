macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < K , V > fmt :: Debug for Drain < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_70!()