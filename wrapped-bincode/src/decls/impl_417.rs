macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < A , B > Encode for (A , B) where A : Encode , B : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_417!()