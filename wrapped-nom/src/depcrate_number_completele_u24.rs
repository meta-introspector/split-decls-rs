// Generated macro for le_u24 (function)
macro_rules! Depcrate_number_completele_u24 {
() => {
// Module: crate::number::complete
// Provides: {"le_u24"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 3 byte integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_u24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u24(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_u24 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { le_uint (input , 3) }
};
}
