macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < T > Encode for Cell < T > where T : Encode + Copy , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (& self . get () , encoder) } }
    };
}

impl_478!()