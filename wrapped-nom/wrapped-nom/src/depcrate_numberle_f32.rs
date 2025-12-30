// Generated macro for le_f32 (function)
macro_rules! Depcrate_numberle_f32 {
() => {
// Module: crate::number
// Provides: {"le_f32"}
// Dependencies: {}
# [doc = " Recognizes a little endian 4 bytes floating point number."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_f32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_f32::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn le_f32 < I , E : ParseError < I > > () -> impl Parser < I , Output = f32 , Error = E > where I : Input < Item = u8 > , { le_u32 () . map (f32 :: from_bits) }
};
}
