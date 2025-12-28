macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < K , V > fmt :: Debug for ValuesMut < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter ()) . finish () } }
    };
}

impl_99!();