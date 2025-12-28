macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Encode for SocketAddrV6 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . ip () . encode (encoder) ? ; self . port () . encode (encoder) } }
    };
}

impl_127!();