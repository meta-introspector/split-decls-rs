// Generated macro for le_i16 (function)
macro_rules! Depcrate_number_completele_i16 {
() => {
// Module: crate::number::complete
// Provides: {"le_i16"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 2 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_i16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i16(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0300)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_i16 < I , E : ParseError < I > > (input : I) -> IResult < I , i16 , E > where I : Input < Item = u8 > , { le_u16 . map (| x | x as i16) . parse (input) }
};
}
