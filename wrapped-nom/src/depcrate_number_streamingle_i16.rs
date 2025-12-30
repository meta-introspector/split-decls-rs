// Generated macro for le_i16 (function)
macro_rules! Depcrate_number_streamingle_i16 {
() => {
// Module: crate::number::streaming
// Provides: {"le_i16"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 2 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_i16;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i16::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"abcd\"[..], 0x0100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_i16 < I , E : ParseError < I > > (input : I) -> IResult < I , i16 , E > where I : Input < Item = u8 > , { le_u16 . map (| x | x as i16) . parse (input) }
};
}
