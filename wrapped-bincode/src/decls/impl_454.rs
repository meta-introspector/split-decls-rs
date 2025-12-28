macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
        IntEncoding!();
        Endianness!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl Encode for i64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_i64 (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } , } } }
    };
}

impl_454!();