macro_rules! deps {
    () => {
        Endianness!();
        Decoder!();
        Decode!();
        DecodeError!();
        IntEncoding!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < Context > Decode < Context > for u64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (8) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_u64 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 8] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => u64 :: from_le_bytes (bytes) , Endianness :: Big => u64 :: from_be_bytes (bytes) , }) } } } }
    };
}

impl_319!()