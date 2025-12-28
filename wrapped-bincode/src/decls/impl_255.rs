macro_rules! deps {
    () => {
        Endianness!();
        IntEncoding!();
        Config!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < T > Config for T where T : InternalEndianConfig + InternalIntEncodingConfig + InternalLimitConfig + Copy + Clone , { fn endianness (& self) -> Endianness { < T as InternalEndianConfig > :: ENDIAN } fn int_encoding (& self) -> IntEncoding { < T as InternalIntEncodingConfig > :: INT_ENCODING } fn limit (& self) -> Option < usize > { < T as InternalLimitConfig > :: LIMIT } }
    };
}

impl_255!();