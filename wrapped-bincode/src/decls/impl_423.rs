macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl < A , B , C , D , E , F , G , H > Encode for (A , B , C , D , E , F , G , H) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , G : Encode , H : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; self . 6 . encode (encoder) ? ; self . 7 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_423!();