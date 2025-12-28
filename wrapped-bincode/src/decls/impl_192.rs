macro_rules! deps {
    () => {
        Encode!();
        SerdeEncoder!();
        EncodeError!();
        Encoder!();
        BorrowCompat!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T > crate :: Encode for BorrowCompat < T > where T : serde :: Serialize , { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { let serializer = ser :: SerdeEncoder { enc : encoder } ; self . 0 . serialize (serializer) ? ; Ok (()) } }
    };
}

impl_192!();