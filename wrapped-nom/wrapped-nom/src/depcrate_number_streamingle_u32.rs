// Generated macro for le_u32 (function)
macro_rules! Depcrate_number_streamingle_u32 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u32"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 4 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u32::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03abcd\"[..]), Ok((&b\"abcd\"[..], 0x03020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn le_u32 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { le_uint (input , 4) }
};
}
