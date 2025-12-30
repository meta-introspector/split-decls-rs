// Generated macro for be_i16 (function)
macro_rules! Depcrate_number_streamingbe_i16 {
() => {
// Module: crate::number::streaming
// Provides: {"be_i16"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 2 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_i16;"] # [doc = ""] # [doc = " let parser = be_i16::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn be_i16 < I , E : ParseError < I > > (input : I) -> IResult < I , i16 , E > where I : Input < Item = u8 > , { be_u16 . map (| x | x as i16) . parse (input) }
};
}
