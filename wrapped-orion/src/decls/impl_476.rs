macro_rules! deps {
    () => {
        PkeParameters!();
        DecapKey!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > core :: fmt :: Debug for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{} {{***OMITTED***}}" , stringify ! ($ name)) } }
    };
}

impl_476!();