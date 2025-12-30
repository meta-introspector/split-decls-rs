// Generated macro for be_u8 (function)
macro_rules! Depcrate_numberbe_u8 {
() => {
// Module: crate::number
// Provides: {"be_u8"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 1 byte integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_u8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u8::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn be_u8 < I , E : ParseError < I > > () -> impl Parser < I , Output = u8 , Error = E > where I : Input < Item = u8 > , { be_uint (1) }
};
}
