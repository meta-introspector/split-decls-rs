macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < K , V : Debug > fmt :: Debug for ValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , val) | val)) . finish () } }
    };
}

impl_323!();