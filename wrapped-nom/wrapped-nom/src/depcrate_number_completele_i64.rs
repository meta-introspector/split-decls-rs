// Generated macro for le_i64 (function)
macro_rules! Depcrate_number_completele_i64 {
() => {
// Module: crate::number::complete
// Provides: {"le_i64"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 8 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_i64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i64(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_i64 < I , E : ParseError < I > > (input : I) -> IResult < I , i64 , E > where I : Input < Item = u8 > , { le_u64 . map (| x | x as i64) . parse (input) }
};
}
