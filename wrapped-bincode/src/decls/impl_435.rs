macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl Encode for bool { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { u8 :: from (* self) . encode (encoder) } }
    };
}

impl_435!()