macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl Encode for char { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encode_utf8 (encoder . writer () , * self) } }
    };
}

impl_464!()