macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Endianness!();
        Encode!();
        IntEncoding!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl Encode for u16 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_u16 (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } , } } }
    };
}

impl_438!();