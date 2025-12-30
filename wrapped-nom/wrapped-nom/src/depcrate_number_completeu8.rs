// Generated macro for u8 (function)
macro_rules! Depcrate_number_completeu8 {
() => {
// Module: crate::number::complete
// Provides: {"u8"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 1 byte integer"] # [doc = ""] # [doc = " Note that endianness does not apply to 1 byte numbers."] # [doc = " *complete version*: returns an error if there is not enough input data"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::u8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   u8(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"\\x03abcefg\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn u8 < I , E : ParseError < I > > (input : I) -> IResult < I , u8 , E > where I : Input < Item = u8 > , { super :: u8 () . parse_complete (input) }
};
}
