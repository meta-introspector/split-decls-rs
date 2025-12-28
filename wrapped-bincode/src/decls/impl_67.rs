macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > Encode for Cow < '_ , T > where T : ToOwned + ? Sized , for < 'a > & 'a T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_ref () . encode (encoder) } }
    };
}

impl_67!()