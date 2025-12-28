macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
        IntEncoding!();
        Endianness!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl Encode for u64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_u64 (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } , } } }
    };
}

impl_442!();