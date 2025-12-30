// Generated macro for be_f32 (function)
macro_rules! Depcrate_number_streamingbe_f32 {
() => {
// Module: crate::number::streaming
// Provides: {"be_f32"}
// Dependencies: {}
# [doc = " Recognizes a big endian 4 bytes floating point number."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_f32;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   be_f32::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00][..]), Ok((&b\"\"[..], 2.640625)));"] # [doc = " assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));"] # [doc = " ```"] # [inline] pub fn be_f32 < I , E : ParseError < I > > (input : I) -> IResult < I , f32 , E > where I : Input < Item = u8 > , { match be_u32 (input) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , f32 :: from_bits (o))) , } }
};
}
