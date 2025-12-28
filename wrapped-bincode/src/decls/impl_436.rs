macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl Encode for u8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& [* self]) } }
    };
}

impl_436!();