// Generated macro for be_u128 (function)
macro_rules! Depcrate_number_completebe_u128 {
() => {
// Module: crate::number::complete
// Provides: {"be_u128"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 16 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::be_u128;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u128(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x00010203040506070001020304050607)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn be_u128 < I , E : ParseError < I > > (input : I) -> IResult < I , u128 , E > where I : Input < Item = u8 > , { be_uint (input , 16) }
};
}
