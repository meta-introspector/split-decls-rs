// Generated macro for be_i32 (function)
macro_rules! Depcrate_number_streamingbe_i32 {
() => {
// Module: crate::number::streaming
// Provides: {"be_i32"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 4 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_i32;"] # [doc = ""] # [doc = " let parser = be_i32::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x00010203)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(4))));"] # [doc = " ```"] # [inline] pub fn be_i32 < I , E : ParseError < I > > (input : I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { be_u32 . map (| x | x as i32) . parse (input) }
};
}
