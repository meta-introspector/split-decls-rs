// Generated macro for be_u24 (function)
macro_rules! Depcrate_number_streamingbe_u24 {
() => {
// Module: crate::number::streaming
// Provides: {"be_u24"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 3 byte integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_u24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u24::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x000102)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn be_u24 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { be_uint (input , 3) }
};
}
