// Generated macro for be_i8 (function)
macro_rules! Depcrate_numberbe_i8 {
() => {
// Module: crate::number
// Provides: {"be_i8"}
// Dependencies: {}
# [doc = " Recognizes a signed 1 byte integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i8;"] # [doc = ""] # [doc = " let mut parser = be_i8::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn be_i8 < I , E : ParseError < I > > () -> impl Parser < I , Output = i8 , Error = E > where I : Input < Item = u8 > , { be_u8 () . map (| x | x as i8) }
};
}
