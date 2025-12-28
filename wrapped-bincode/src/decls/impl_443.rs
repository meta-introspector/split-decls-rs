macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl Encode for NonZeroU64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
    };
}

impl_443!();