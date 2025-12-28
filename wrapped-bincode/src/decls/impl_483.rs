macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < T > Encode for Bound < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { Self :: Unbounded => { 0u32 . encode (encoder) ? ; } Self :: Included (val) => { 1u32 . encode (encoder) ? ; val . encode (encoder) ? ; } Self :: Excluded (val) => { 2u32 . encode (encoder) ? ; val . encode (encoder) ? ; } } Ok (()) } }
    };
}

impl_483!();