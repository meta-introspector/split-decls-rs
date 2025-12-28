macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl Encode for Duration { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_secs () . encode (encoder) ? ; self . subsec_nanos () . encode (encoder) ? ; Ok (()) } }
    };
}

impl_480!()