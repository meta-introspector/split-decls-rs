macro_rules! deps {
    () => {
        LittleEndian!();
        Endianness!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl InternalEndianConfig for LittleEndian { const ENDIAN : Endianness = Endianness :: Little ; }
    };
}

impl_259!()