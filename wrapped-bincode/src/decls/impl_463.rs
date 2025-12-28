macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < T : Encode > Encode for Reverse < T > { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . 0 . encode (encoder) } }
    };
}

impl_463!()