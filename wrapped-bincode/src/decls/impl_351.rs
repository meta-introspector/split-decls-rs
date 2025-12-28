macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        Endianness!();
        DecodeError!();
        IntEncoding!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < Context > Decode < Context > for isize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (8) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_isize (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 8] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => i64 :: from_le_bytes (bytes) , Endianness :: Big => i64 :: from_be_bytes (bytes) , } as isize) } } } }
    };
}

impl_351!();