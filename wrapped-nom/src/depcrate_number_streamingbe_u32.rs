// Generated macro for be_u32 (function)
macro_rules! Depcrate_number_streamingbe_u32 {
() => {
// Module: crate::number::streaming
// Provides: {"be_u32"}
// Dependencies: {}
# [doc = " Recognizes a big endian unsigned 4 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_u32::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x00010203)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn be_u32 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { be_uint (input , 4) }
};
}
