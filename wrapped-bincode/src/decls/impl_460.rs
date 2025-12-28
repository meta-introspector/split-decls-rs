macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
        Endianness!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl Encode for f32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } } }
    };
}

impl_460!();