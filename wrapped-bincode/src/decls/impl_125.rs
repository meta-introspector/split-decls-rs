macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < Context > Decode < Context > for SocketAddrV4 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let ip = Ipv4Addr :: decode (decoder) ? ; let port = u16 :: decode (decoder) ? ; Ok (Self :: new (ip , port)) } }
    };
}

impl_125!();