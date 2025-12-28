macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T > Encode for Rc < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
    };
}

impl_73!();