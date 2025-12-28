macro_rules! deps {
    () => {
        Endianness!();
        DecodeError!();
        IntEncoding!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < Context > Decode < Context > for usize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (8) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_usize (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 8] ; decoder . reader () . read (& mut bytes) ? ; let value = match D :: C :: ENDIAN { Endianness :: Little => u64 :: from_le_bytes (bytes) , Endianness :: Big => u64 :: from_be_bytes (bytes) , } ; value . try_into () . map_err (| _ | DecodeError :: OutsideUsizeRange (value)) } } } }
    };
}

impl_327!()