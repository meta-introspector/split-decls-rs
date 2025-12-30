// Generated macro for be_i64 (function)
macro_rules! Depcrate_number_streamingbe_i64 {
() => {
// Module: crate::number::streaming
// Provides: {"be_i64"}
// Dependencies: {}
# [doc = " Recognizes a big endian signed 8 bytes integer."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " use nom::number::streaming::be_i64;"] # [doc = ""] # [doc = " let parser = be_i64::<_, (_, ErrorKind)>;"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07abcd\"[..]), Ok((&b\"abcd\"[..], 0x0001020304050607)));"] # [doc = " assert_eq!(parser(&b\"\\x01\"[..]), Err(Err::Incomplete(Needed::new(7))));"] # [doc = " ```"] # [inline] pub fn be_i64 < I , E : ParseError < I > > (input : I) -> IResult < I , i64 , E > where I : Input < Item = u8 > , { be_u64 . map (| x | x as i64) . parse (input) }
};
}
