macro_rules! deps {
    () => {
        Decoder!();
        Endianness!();
        Decode!();
        IntEncoding!();
        DecodeError!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < Context > Decode < Context > for i128 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (16) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_i128 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 16] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => i128 :: from_le_bytes (bytes) , Endianness :: Big => i128 :: from_be_bytes (bytes) , }) } } } }
    };
}

impl_347!();