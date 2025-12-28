macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Encode for SocketAddr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { SocketAddr :: V4 (v4) => { 0u32 . encode (encoder) ? ; v4 . encode (encoder) } SocketAddr :: V6 (v6) => { 1u32 . encode (encoder) ? ; v6 . encode (encoder) } } } }
    };
}

impl_121!();