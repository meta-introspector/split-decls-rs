macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Endianness!();
        Decode!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < Context > Decode < Context > for f32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (4) ? ; let mut bytes = [0u8 ; 4] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => f32 :: from_le_bytes (bytes) , Endianness :: Big => f32 :: from_be_bytes (bytes) , }) } }
    };
}

impl_355!()