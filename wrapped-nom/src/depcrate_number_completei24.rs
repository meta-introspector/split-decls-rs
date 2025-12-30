// Generated macro for i24 (function)
macro_rules! Depcrate_number_completei24 {
() => {
// Module: crate::number::complete
// Provides: {"i24"}
// Dependencies: {}
# [doc = " Recognizes a signed 3 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian i24 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian i24 integer."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::i24;"] # [doc = ""] # [doc = " let be_i24 = |s| {"] # [doc = "   i24(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_i24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x000305)));"] # [doc = " assert_eq!(be_i24(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = ""] # [doc = " let le_i24 = |s| {"] # [doc = "   i24(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_i24(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(le_i24(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn i24 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { move | input | super :: i24 (endian) . parse_complete (input) }
};
}
