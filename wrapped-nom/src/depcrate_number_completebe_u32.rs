// Generated macro for be_u32 (function)
macro_rules! Depcrate_number_completebe_u32 {
() => {
// Module: crate::number::complete
// Provides: {"be_u32"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 4 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::be_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u32(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03\\x05\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x00030507)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn be_u32 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { be_uint (input , 4) }
};
}
