// Generated macro for le_f64 (function)
macro_rules! Depcrate_number_completele_f64 {
() => {
// Module: crate::number::complete
// Provides: {"le_f64"}
// Dependencies: {}
# [doc = " Recognizes a little endian 8 bytes floating point number."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::le_f64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_f64(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(parser(&b\"abc\"[..]), Err(Err::Error((&b\"abc\"[..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn le_f64 < I , E : ParseError < I > > (input : I) -> IResult < I , f64 , E > where I : Input < Item = u8 > , { match le_u64 (input) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , f64 :: from_bits (o))) , } }
};
}
