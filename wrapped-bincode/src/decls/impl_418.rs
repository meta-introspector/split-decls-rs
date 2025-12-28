macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl < A , B , C > Encode for (A , B , C) where A : Encode , B : Encode , C : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_418!()