// Generated macro for be_i32 (function)
macro_rules! Depcrate_numberbe_i32 {
() => {
// Module: crate::number
// Provides: {"be_i32"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 4 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i32;"] # [doc = ""] # [doc = " let mut parser = be_i32::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x00010203)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(4))));"] # [doc = " ```"] # [inline] pub fn be_i32 < I , E : ParseError < I > > () -> impl Parser < I , Output = i32 , Error = E > where I : Input < Item = u8 > , { be_u32 () . map (| x | x as i32) }
};
}
