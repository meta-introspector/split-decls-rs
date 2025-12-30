// Generated macro for le_u128 (function)
macro_rules! Depcrate_number_streamingle_u128 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u128"}
// Dependencies: {}
# [doc = " Recognizes a little endian unsigned 16 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u128;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   le_u128::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x10\\x11\\x12\\x13\\x14\\x15abcd\"[..]), Ok((&b\"abcd\"[..], 0x15141312111009080706050403020100)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(15))));"] # [doc = " ```"] # [inline] pub fn le_u128 < I , E : ParseError < I > > (input : I) -> IResult < I , u128 , E > where I : Input < Item = u8 > , { le_uint (input , 16) }
};
}
