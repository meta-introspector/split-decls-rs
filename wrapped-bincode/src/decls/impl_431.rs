macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < A , B , C , D , E , F , G , H , I , J , K , L , M , N , O , P > Encode for (A , B , C , D , E , F , G , H , I , J , K , L , M , N , O , P) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , G : Encode , H : Encode , I : Encode , J : Encode , K : Encode , L : Encode , M : Encode , N : Encode , O : Encode , P : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; self . 6 . encode (encoder) ? ; self . 7 . encode (encoder) ? ; self . 8 . encode (encoder) ? ; self . 9 . encode (encoder) ? ; self . 10 . encode (encoder) ? ; self . 11 . encode (encoder) ? ; self . 12 . encode (encoder) ? ; self . 13 . encode (encoder) ? ; self . 14 . encode (encoder) ? ; self . 15 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_431!()