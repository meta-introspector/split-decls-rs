// Generated macro for be_f32 (function)
macro_rules! Depcrate_number_completebe_f32 {
() => {
// Module: crate::number::complete
// Provides: {"be_f32"}
// Dependencies: {}
# [doc = " Recognizes a big endian 4 bytes floating point number."] # [doc = ""] # [doc = " *Complete version*: Returns an error if there is not enough input data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::be_f32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_f32(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b\"\"[..], 12.5)));"] # [doc = " assert_eq!(parser(&b\"abc\"[..]), Err(Err::Error((&b\"abc\"[..], ErrorKind::Eof))));"] # [doc = " ```"] # [inline] pub fn be_f32 < I , E : ParseError < I > > (input : I) -> IResult < I , f32 , E > where I : Input < Item = u8 > , { match be_u32 (input) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , f32 :: from_bits (o))) , } }
};
}
