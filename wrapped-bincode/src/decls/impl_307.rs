macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < Context > Decode < Context > for u8 { # [inline] fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (1) ? ; if let Some (buf) = decoder . reader () . peek_read (1) { let byte = buf [0] ; decoder . reader () . consume (1) ; Ok (byte) } else { let mut bytes = [0u8 ; 1] ; decoder . reader () . read (& mut bytes) ? ; Ok (bytes [0]) } } }
    };
}

impl_307!();