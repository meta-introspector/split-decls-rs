// Generated macro for i16 (function)
macro_rules! Depcrate_number_streamingi16 {
() => {
// Module: crate::number::streaming
// Provides: {"i16"}
// Dependencies: {}
# [doc = " Recognizes a signed 2 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian i16 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian i16 integer."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::streaming::i16;"] # [doc = ""] # [doc = " let be_i16 = |s| {"] # [doc = "   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_i16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0003)));"] # [doc = " assert_eq!(be_i16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = ""] # [doc = " let le_i16 = |s| {"] # [doc = "   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_i16(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0300)));"] # [doc = " assert_eq!(le_i16(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn i16 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , i16 , E > where I : Input < Item = u8 > , { move | input | super :: i16 (endian) . parse (input) }
};
}
