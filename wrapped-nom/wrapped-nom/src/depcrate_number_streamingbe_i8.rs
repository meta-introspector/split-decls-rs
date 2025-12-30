// Generated macro for be_i8 (function)
macro_rules! Depcrate_number_streamingbe_i8 {
() => {
// Module: crate::number::streaming
// Provides: {"be_i8"}
// Dependencies: {}
# [doc = " Recognizes a signed 1 byte integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_i8;"] # [doc = ""] # [doc = " let parser = be_i8::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn be_i8 < I , E : ParseError < I > > (input : I) -> IResult < I , i8 , E > where I : Input < Item = u8 > , { be_u8 . map (| x | x as i8) . parse (input) }
};
}
