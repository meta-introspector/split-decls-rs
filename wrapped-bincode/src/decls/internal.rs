macro_rules! deps {
    () => {
        Endianness!();
        Configuration!();
        IntEncoding!();
    };
}

macro_rules! internal {
    () => {
        deps!();
        mod internal { use super :: { Configuration , Endianness , IntEncoding } ; pub trait InternalEndianConfig { const ENDIAN : Endianness ; } impl < E : InternalEndianConfig , I , L > InternalEndianConfig for Configuration < E , I , L > { const ENDIAN : Endianness = E :: ENDIAN ; } pub trait InternalIntEncodingConfig { const INT_ENCODING : IntEncoding ; } impl < E , I : InternalIntEncodingConfig , L > InternalIntEncodingConfig for Configuration < E , I , L > { const INT_ENCODING : IntEncoding = I :: INT_ENCODING ; } pub trait InternalLimitConfig { const LIMIT : Option < usize > ; } impl < E , I , L : InternalLimitConfig > InternalLimitConfig for Configuration < E , I , L > { const LIMIT : Option < usize > = L :: LIMIT ; } }
    };
}

internal!();