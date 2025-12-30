// Generated macro for be_u16 (function)
macro_rules! Depcrate_number_streamingbe_u16 {
() => {
// Module: crate::number::streaming
// Provides: {"be_u16"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 2 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_u16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u16::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn be_u16 < I , E : ParseError < I > > (input : I) -> IResult < I , u16 , E > where I : Input < Item = u8 > , { be_uint (input , 2) }
};
}
