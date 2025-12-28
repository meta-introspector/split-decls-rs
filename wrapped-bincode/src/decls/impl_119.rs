macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < Context > Decode < Context > for Ipv6Addr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let mut buff = [0u8 ; 16] ; decoder . reader () . read (& mut buff) ? ; Ok (Self :: from (buff)) } }
    };
}

impl_119!();