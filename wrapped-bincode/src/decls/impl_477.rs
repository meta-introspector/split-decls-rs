macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        impl < T , U > Encode for Result < T , U > where T : Encode , U : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { Ok (val) => { 0u32 . encode (encoder) ? ; val . encode (encoder) } Err (err) => { 1u32 . encode (encoder) ? ; err . encode (encoder) } } } }
    };
}

impl_477!()