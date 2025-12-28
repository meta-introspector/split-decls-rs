macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl Encode for NonZeroU8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
    };
}

impl_437!();