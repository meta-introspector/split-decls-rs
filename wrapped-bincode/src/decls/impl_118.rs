macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Encode for Ipv6Addr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& self . octets ()) } }
    };
}

impl_118!()