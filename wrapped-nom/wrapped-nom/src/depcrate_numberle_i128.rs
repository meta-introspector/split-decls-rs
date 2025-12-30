// Generated macro for le_i128 (function)
macro_rules! Depcrate_numberle_i128 {
() => {
// Module: crate::number
// Provides: {"le_i128"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 16 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_i128;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i128::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x10\\x11\\x12\\x13\\x14\\x15abcd\"[..]), Ok((&b\"abcd\"[..], 0x15141312111009080706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(15))));"] # [doc = " ```"] # [inline] pub fn le_i128 < I , E : ParseError < I > > () -> impl Parser < I , Output = i128 , Error = E > where I : Input < Item = u8 > , { le_u128 () . map (| x | x as i128) }
};
}
