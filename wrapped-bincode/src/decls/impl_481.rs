macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        impl < T > Encode for Range < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . start . encode (encoder) ? ; self . end . encode (encoder) ? ; Ok (()) } }
    };
}

impl_481!();