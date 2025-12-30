// Generated macro for le_f64 (function)
macro_rules! Depcrate_numberle_f64 {
() => {
// Module: crate::number
// Provides: {"le_f64"}
// Dependencies: {}
# [doc = " Recognizes a little endian 8 bytes floating point number."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::le_f64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_f64::<_, (_, ErrorKind)>().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x41][..]), Ok((&b\"\"[..], 3145728.0)));"] # [doc = " assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn le_f64 < I , E : ParseError < I > > () -> impl Parser < I , Output = f64 , Error = E > where I : Input < Item = u8 > , { le_u64 () . map (f64 :: from_bits) }
};
}
