// Generated macro for le_u8 (function)
macro_rules! Depcrate_number_streamingle_u8 {
() => {
// Module: crate::number::streaming
// Provides: {"le_u8"}
// Dependencies: {}
# [doc = " Recognizes an unsigned 1 byte integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::le_u8;"] # [doc = ""] # [doc = " let parser = le_u8::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01abcd\"[..]), Ok((&b\"\\x01abcd\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn le_u8 < I , E : ParseError < I > > (input : I) -> IResult < I , u8 , E > where I : Input < Item = u8 > , { le_uint (input , 1) }
};
}
