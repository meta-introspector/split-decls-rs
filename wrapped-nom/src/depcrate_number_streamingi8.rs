// Generated macro for i8 (function)
macro_rules! Depcrate_number_streamingi8 {
() => {
// Module: crate::number::streaming
// Provides: {"i8"}
// Dependencies: {}
# [doc = " Recognizes a signed 1 byte integer"] # [doc = ""] # [doc = " Note that endianness does not apply to 1 byte numbers."] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::streaming::i8;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   i8::<_, (_, ErrorKind)>(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x03abcefg\"[..]), Ok((&b\"\\x03abcefg\"[..], 0x00)));"] # [doc = " assert_eq!(parser(&b\"\"[..]), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [inline] pub fn i8 < I , E : ParseError < I > > (i : I) -> IResult < I , i8 , E > where I : Input < Item = u8 > , { super :: u8 () . map (| x | x as i8) . parse (i) }
};
}
