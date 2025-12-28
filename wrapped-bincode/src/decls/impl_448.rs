macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl Encode for i8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& [* self as u8]) } }
    };
}

impl_448!();