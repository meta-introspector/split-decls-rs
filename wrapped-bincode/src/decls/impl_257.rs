macro_rules! deps {
    () => {
        BigEndian!();
        Endianness!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl InternalEndianConfig for BigEndian { const ENDIAN : Endianness = Endianness :: Big ; }
    };
}

impl_257!();