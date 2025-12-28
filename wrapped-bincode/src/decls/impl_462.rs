macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < T : Encode > Encode for Wrapping < T > { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . 0 . encode (encoder) } }
    };
}

impl_462!();