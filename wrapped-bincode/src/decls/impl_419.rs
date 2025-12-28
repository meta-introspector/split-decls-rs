macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl < A , B , C , D > Encode for (A , B , C , D) where A : Encode , B : Encode , C : Encode , D : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_419!();