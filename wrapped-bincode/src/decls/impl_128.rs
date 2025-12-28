macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < Context > Decode < Context > for SocketAddrV6 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let ip = Ipv6Addr :: decode (decoder) ? ; let port = u16 :: decode (decoder) ? ; Ok (Self :: new (ip , port , 0 , 0)) } }
    };
}

impl_128!();