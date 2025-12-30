// Generated macro for u24 (function)
macro_rules! Depcrate_number_completeu24 {
() => {
// Module: crate::number::complete
// Provides: {"u24"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 3 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u24 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u24 integer."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::u24;"] # [doc = ""] # [doc = " let be_u24 = |s| {"] # [doc = "   u24(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x000305)));"] # [doc = " assert_eq!(be_u24(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = ""] # [doc = " let le_u24 = |s| {"] # [doc = "   u24(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(le_u24(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn u24 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { move | input | super :: u24 (endian) . parse_complete (input) }
};
}
