// Generated macro for le_i128 (function)
macro_rules! Depcrate_number_completele_i128 {
() => {
// Module: crate::number::complete
// Provides: {"le_i128"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 16 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_i128;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i128(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x07060504030201000706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_i128 < I , E : ParseError < I > > (input : I) -> IResult < I , i128 , E > where I : Input < Item = u8 > , { le_u128 . map (| x | x as i128) . parse (input) }
};
}
