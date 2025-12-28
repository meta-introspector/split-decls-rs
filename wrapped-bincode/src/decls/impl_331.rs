macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < Context > Decode < Context > for i8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (1) ? ; let mut bytes = [0u8 ; 1] ; decoder . reader () . read (& mut bytes) ? ; Ok (bytes [0] as i8) } }
    };
}

impl_331!()