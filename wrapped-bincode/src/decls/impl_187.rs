macro_rules! deps {
    () => {
        Encoder!();
        Compat!();
        Encode!();
        EncodeError!();
        SerdeEncoder!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < T > crate :: Encode for Compat < T > where T : serde :: Serialize , { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { let serializer = ser :: SerdeEncoder { enc : encoder } ; self . 0 . serialize (serializer) ? ; Ok (()) } }
    };
}

impl_187!()