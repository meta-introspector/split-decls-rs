macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl Encode for NonZeroU32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
    };
}

impl_441!();