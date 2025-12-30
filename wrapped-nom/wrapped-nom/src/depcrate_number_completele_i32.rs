// Generated macro for le_i32 (function)
macro_rules! Depcrate_number_completele_i32 {
() => {
// Module: crate::number::complete
// Provides: {"le_i32"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 4 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_i32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i32(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x07050300)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_i32 < I , E : ParseError < I > > (input : I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { le_u32 . map (| x | x as i32) . parse (input) }
};
}
