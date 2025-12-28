macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl Encode for NonZeroI32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
    };
}

impl_453!();