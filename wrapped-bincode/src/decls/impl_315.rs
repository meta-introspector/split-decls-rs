macro_rules! deps {
    () => {
        Endianness!();
        Decode!();
        Decoder!();
        DecodeError!();
        IntEncoding!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < Context > Decode < Context > for u32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (4) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_u32 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 4] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => u32 :: from_le_bytes (bytes) , Endianness :: Big => u32 :: from_be_bytes (bytes) , }) } } } }
    };
}

impl_315!()