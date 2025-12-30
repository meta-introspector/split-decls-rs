// Generated macro for be_u8 (function)
macro_rules! Depcrate_number_completebe_u8 {
() => {
// Module: crate::number::complete
// Provides: {"be_u8"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 1 byte integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::be_u8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u8(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"\\x03abcefg\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn be_u8 < I , E : ParseError < I > > (input : I) -> IResult < I , u8 , E > where I : Input < Item = u8 > , { be_uint (input , 1) }
};
}
