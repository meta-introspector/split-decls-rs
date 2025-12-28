macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl Encode for NonZeroIsize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
    };
}

impl_459!();