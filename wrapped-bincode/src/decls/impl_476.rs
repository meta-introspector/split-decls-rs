macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < T > Encode for Option < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { super :: encode_option_variant (encoder , self) ? ; if let Some (val) = self { val . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_476!();