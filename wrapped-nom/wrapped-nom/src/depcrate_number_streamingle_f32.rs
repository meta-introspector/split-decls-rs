// Generated macro for le_f32 (function)
macro_rules! Depcrate_number_streamingle_f32 {
() => {
// Module: crate::number::streaming
// Provides: {"le_f32"}
// Dependencies: {}
# [doc = " Recognizes a little endian 4 bytes floating point number."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_f32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_f32::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn le_f32 < I , E : ParseError < I > > (input : I) -> IResult < I , f32 , E > where I : Input < Item = u8 > , { match le_u32 (input) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , f32 :: from_bits (o))) , } }
};
}
