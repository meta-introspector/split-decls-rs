macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < Context > Decode < Context > for Ipv4Addr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let mut buff = [0u8 ; 4] ; decoder . reader () . read (& mut buff) ? ; Ok (Self :: from (buff)) } }
    };
}

impl_116!()