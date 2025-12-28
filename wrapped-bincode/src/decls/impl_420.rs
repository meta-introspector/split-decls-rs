macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < A , B , C , D , E > Encode for (A , B , C , D , E) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_420!()