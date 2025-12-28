macro_rules! deps {
    () => {
        Endianness!();
        IntEncoding!();
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl Encode for usize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_usize (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& (* self as u64) . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& (* self as u64) . to_le_bytes ()) , } , } } }
    };
}

impl_446!();