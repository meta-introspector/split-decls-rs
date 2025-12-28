macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        IntEncoding!();
        Endianness!();
        Encode!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl Encode for i128 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_i128 (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } , } } }
    };
}

impl_456!()