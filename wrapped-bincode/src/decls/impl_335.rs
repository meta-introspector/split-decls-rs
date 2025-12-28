macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
        IntEncoding!();
        Endianness!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < Context > Decode < Context > for i16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (2) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_i16 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 2] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => i16 :: from_le_bytes (bytes) , Endianness :: Big => i16 :: from_be_bytes (bytes) , }) } } } }
    };
}

impl_335!()