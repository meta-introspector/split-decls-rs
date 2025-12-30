// Generated macro for i32 (function)
macro_rules! Depcrate_number_completei32 {
() => {
// Module: crate::number::complete
// Provides: {"i32"}
// Dependencies: {}
# [doc = " Recognizes a signed 4 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian i32 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian i32 integer."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::i32;"] # [doc = ""] # [doc = " let be_i32 = |s| {"] # [doc = "   i32(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_i32(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x00030507)));"] # [doc = " assert_eq!(be_i32(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = ""] # [doc = " let le_i32 = |s| {"] # [doc = "   i32(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_i32(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x07050300)));"] # [doc = " assert_eq!(le_i32(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn i32 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { move | input | super :: i32 (endian) . parse_complete (input) }
};
}
