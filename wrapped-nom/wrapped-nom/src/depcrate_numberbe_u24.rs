// Generated macro for be_u24 (function)
macro_rules! Depcrate_numberbe_u24 {
() => {
// Module: crate::number
// Provides: {"be_u24"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 3 byte integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_u24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u24::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x000102)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn be_u24 < I , E : ParseError < I > > () -> impl Parser < I , Output = u32 , Error = E > where I : Input < Item = u8 > , { be_uint (3) }
};
}
