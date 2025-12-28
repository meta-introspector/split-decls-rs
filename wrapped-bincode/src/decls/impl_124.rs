macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Encode for SocketAddrV4 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . ip () . encode (encoder) ? ; self . port () . encode (encoder) } }
    };
}

impl_124!()