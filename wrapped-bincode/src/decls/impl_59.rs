macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Encode for String { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_bytes () . encode (encoder) } }
    };
}

impl_59!();