// Generated macro for be_i24 (function)
macro_rules! Depcrate_number_streamingbe_i24 {
() => {
// Module: crate::number::streaming
// Provides: {"be_i24"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 3 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_i24;"] # [doc = ""] # [doc = " let parser = be_i24::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x000102)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn be_i24 < I , E : ParseError < I > > (input : I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { be_u24 . map (| x | { if x & 0x80_00_00 != 0 { (x | 0xff_00_00_00) as i32 } else { x as i32 } }) . parse (input) }
};
}
