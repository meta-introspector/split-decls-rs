// Generated macro for u32 (function)
macro_rules! Depcrate_number_streamingu32 {
() => {
// Module: crate::number::streaming
// Provides: {"u32"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 4 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u32 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u32 integer."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::streaming::u32;"] # [doc = ""] # [doc = " let be_u32 = |s| {"] # [doc = "   u32::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u32(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x00030507)));"] # [doc = " assert_eq!(be_u32(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = ""] # [doc = " let le_u32 = |s| {"] # [doc = "   u32::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u32(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x07050300)));"] # [doc = " assert_eq!(le_u32(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn u32 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { move | input | super :: u32 (endian) . parse (input) }
};
}
