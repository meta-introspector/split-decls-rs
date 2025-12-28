macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
        IntEncoding!();
        Endianness!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < Context > Decode < Context > for i32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (4) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_i32 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 4] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => i32 :: from_le_bytes (bytes) , Endianness :: Big => i32 :: from_be_bytes (bytes) , }) } } } }
    };
}

impl_339!()