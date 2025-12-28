macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < A , B , C , D , E , F > Encode for (A , B , C , D , E , F) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_421!();