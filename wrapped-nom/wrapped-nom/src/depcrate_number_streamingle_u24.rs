// Generated macro for le_u24 (function)
macro_rules! Depcrate_number_streamingle_u24 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u24"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 3 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u24::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn le_u24 < I , E : ParseError < I > > (input : I) -> IResult < I , u32 , E > where I : Input < Item = u8 > , { le_uint (input , 3) }
};
}
