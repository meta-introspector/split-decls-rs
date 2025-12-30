// Generated macro for u16 (function)
macro_rules! Depcrate_number_streamingu16 {
() => {
// Module: crate::number::streaming
// Provides: {"u16"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 2 bytes integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u16 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u16 integer."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::streaming::u16;"] # [doc = ""] # [doc = " let be_u16 = |s| {"] # [doc = "   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0003)));"] # [doc = " assert_eq!(be_u16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = ""] # [doc = " let le_u16 = |s| {"] # [doc = "   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0300)));"] # [doc = " assert_eq!(le_u16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn u16 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , u16 , E > where I : Input < Item = u8 > , { move | input | super :: u16 (endian) . parse (input) }
};
}
