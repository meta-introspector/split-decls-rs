macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < T > Encode for & T where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
    };
}

impl_484!();