// Generated macro for take_until1 (function)
macro_rules! Depcrate_bytestake_until1 {
() => {
// Module: crate::bytes
// Provides: {"take_until1"}
// Dependencies: {}
# [doc = " Returns the non empty input slice up to the first occurrence of the pattern."] # [doc = ""] # [doc = " It doesn't consume the pattern."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't"] # [doc = " contain the pattern or if the input is smaller than the pattern."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_until1;"] # [doc = ""] # [doc = " fn until_eof(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_until1(\"eof\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(until_eof(\"hello, worldeof\"), Ok((\"eof\", \"hello, world\")));"] # [doc = " assert_eq!(until_eof(\"hello, world\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(until_eof(\"hello, worldeo\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " assert_eq!(until_eof(\"1eof2eof\"), Ok((\"eof2eof\", \"1\")));"] # [doc = " assert_eq!(until_eof(\"eof\"),  Err(Err::Error(Error::new(\"eof\", ErrorKind::TakeUntil))));"] # [doc = " ```"] pub fn take_until1 < T , I , Error : ParseError < I > > (tag : T) -> impl Parser < I , Output = I , Error = Error > where I : Input + FindSubstring < T > , T : Clone , { TakeUntil1 { tag , e : PhantomData , } }
};
}
