macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl Encode for & CStr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . to_bytes () . encode (encoder) } }
    };
}

impl_94!()