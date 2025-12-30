// Generated macro for le_i24 (function)
macro_rules! Depcrate_number_streamingle_i24 {
() => {
// Module: crate::number::streaming
// Provides: {"le_i24"}
// Dependencies: {}
# [doc = " Recognizes a little endian signed 3 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_i24;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_i24::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02abcd\"[..]), Ok((&b\"abcd\"[..], 0x020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] # [inline] pub fn le_i24 < I , E : ParseError < I > > (input : I) -> IResult < I , i32 , E > where I : Input < Item = u8 > , { le_u24 . map (| x | { if x & 0x80_00_00 != 0 { (x | 0xff_00_00_00) as i32 } else { x as i32 } }) . parse (input) }
};
}
