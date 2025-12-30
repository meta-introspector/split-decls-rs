// Generated macro for le_i24 (function)
macro_rules! Depcrate_number_completele_i24 {
() => {
// Module: crate::number::complete
// Provides: {"le_i24"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 3 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_i24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i24(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03\\x05abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x050300)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_i24 < I , E : ParseError < I > > (input : I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { le_u24 . map (| x | { if x & 0x80_00_00 != 0 { (x | 0xff_00_00_00) as i32 } else { x as i32 } }) . parse (input) }
};
}
