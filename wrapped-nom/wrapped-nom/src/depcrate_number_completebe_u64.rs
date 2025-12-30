// Generated macro for be_u64 (function)
macro_rules! Depcrate_number_completebe_u64 {
() => {
// Module: crate::number::complete
// Provides: {"be_u64"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 8 bytes integer."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::be_u64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u64(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcefg\"[..]), Ok((&b\"abcefg\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn be_u64 < I , E : ParseError < I > > (input : I) -> IResult < I , u64 , E > where I : Input < Item = u8 > , { be_uint (input , 8) }
};
}
