// Generated macro for be_i24 (function)
macro_rules! Depcrate_numberbe_i24 {
() => {
// Module: crate::number
// Provides: {"be_i24"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 3 bytes integer."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::be_i24;"] # [doc = ""] # [doc = " let mut parser = be_i24::<_, (_, ErrorKind)>();"] # [doc = ""] # [doc = " assert_eq!(parser.parse(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x000102)));"] # [doc = " assert_eq!(parser.parse(&b\"\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn be_i24 < I , E : ParseError < I > > () -> impl Parser < I , Output = i32 , Error = E > where I : Input < Item = u8 > , { be_u24 () . map (| x | { if x & 0x80_00_00 != 0 { (x | 0xff_00_00_00) as i32 } else { x as i32 } }) }
};
}
