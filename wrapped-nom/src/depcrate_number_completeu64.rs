// Generated macro for u64 (function)
macro_rules! Depcrate_number_completeu64 {
() => {
// Module: crate::number::complete
// Provides: {"u64"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 8 byte integer"] # [doc = ""] # [doc = " If the parameter is `nom::number::Endianness::Big`, parse a big endian u64 integer,"] # [doc = " otherwise if `nom::number::Endianness::Little` parse a little endian u64 integer."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::u64;"] # [doc = ""] # [doc = " let be_u64 = |s| {"] # [doc = "   u64(nom::number::Endianness::Big)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(be_u64(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(be_u64(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = ""] # [doc = " let le_u64 = |s| {"] # [doc = "   u64(nom::number::Endianness::Little)(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(le_u64(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0706050403020100)));"] # [doc = " assert_eq!(le_u64(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn u64 < I , E : ParseError < I > > (endian : crate :: number :: Endianness ,) -> impl Fn (I) -> IResult < I , u64 , E > where I : Input < Item = u8 > , { move | input | super :: u64 (endian) . parse_complete (input) }
};
}
