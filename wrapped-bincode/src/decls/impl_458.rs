macro_rules! deps {
    () => {
        Endianness!();
        Encode!();
        Encoder!();
        IntEncoding!();
        EncodeError!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl Encode for isize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_isize (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& (* self as i64) . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& (* self as i64) . to_le_bytes ()) , } , } } }
    };
}

impl_458!();