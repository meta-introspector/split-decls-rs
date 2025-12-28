macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Encode for IpAddr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { IpAddr :: V4 (v4) => { 0u32 . encode (encoder) ? ; v4 . encode (encoder) } IpAddr :: V6 (v6) => { 1u32 . encode (encoder) ? ; v6 . encode (encoder) } } } }
    };
}

impl_112!();