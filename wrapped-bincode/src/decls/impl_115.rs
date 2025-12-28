macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl Encode for Ipv4Addr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& self . octets ()) } }
    };
}

impl_115!();