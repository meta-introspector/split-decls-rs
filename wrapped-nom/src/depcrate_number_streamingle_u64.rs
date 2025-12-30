// Generated macro for le_u64 (function)
macro_rules! Depcrate_number_streamingle_u64 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u64"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 8 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u64;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u64::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcd\"[..]), Ok((&b\"abcd\"[..], 0x0706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn le_u64 < I , E : ParseError < I > > (input : I) -> IResult < I , u64 , E > where I : Input < Item = u8 > , { le_uint (input , 8) }
};
}
