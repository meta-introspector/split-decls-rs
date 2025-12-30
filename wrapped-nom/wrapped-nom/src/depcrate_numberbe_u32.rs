// Generated macro for be_u32 (function)
macro_rules! Depcrate_numberbe_u32 {
() => {
// Module: crate::number
// Provides: {"be_u32"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 4 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u32::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x00010203)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn be_u32 < I , E : ParseError < I > > () -> impl Parser < I , Output = u32 , Error = E > where I : Input < Item = u8 > , { be_uint (4) }
};
}
