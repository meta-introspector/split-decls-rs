macro_rules! deps {
    () => {
        IntEncoding!();
        Endianness!();
    };
}

macro_rules! Config {
    () => {
        deps!();
        # [doc = " Indicates a type is valid for controlling the bincode configuration"] pub trait Config : InternalEndianConfig + InternalIntEncodingConfig + InternalLimitConfig + Copy + Clone { # [doc = " This configuration's Endianness"] fn endianness (& self) -> Endianness ; # [doc = " This configuration's Integer Encoding"] fn int_encoding (& self) -> IntEncoding ; # [doc = " This configuration's byte limit, or `None` if no limit is configured"] fn limit (& self) -> Option < usize > ; }
    };
}

Config!();