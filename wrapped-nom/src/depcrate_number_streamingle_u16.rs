// Generated macro for le_u16 (function)
macro_rules! Depcrate_number_streamingle_u16 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u16"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 2 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u16::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_u16 < I , E : ParseError < I > > (input : I) -> IResult < I , u16 , E > where I : Input < Item = u8 > , { le_uint (input , 2) }
};
}
