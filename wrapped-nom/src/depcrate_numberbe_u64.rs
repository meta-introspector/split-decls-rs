// Generated macro for be_u64 (function)
macro_rules! Depcrate_numberbe_u64 {
() => {
// Module: crate::number
// Provides: {"be_u64"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 8 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_u64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u64::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn be_u64 < I , E : ParseError < I > > () -> impl Parser < I , Output = u64 , Error = E > where I : Input < Item = u8 > , { be_uint (8) }
};
}
