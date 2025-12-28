macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Encode for CString { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_bytes () . encode (encoder) } }
    };
}

impl_95!()