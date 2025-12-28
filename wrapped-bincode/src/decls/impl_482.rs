macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < T > Encode for RangeInclusive < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . start () . encode (encoder) ? ; self . end () . encode (encoder) ? ; Ok (()) } }
    };
}

impl_482!()