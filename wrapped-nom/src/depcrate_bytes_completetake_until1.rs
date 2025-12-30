// Generated macro for take_until1 (function)
macro_rules! Depcrate_bytes_completetake_until1 {
() => {
// Module: crate::bytes::complete
// Provides: {"take_until1"}
// Dependencies: {}
# [doc = " Returns the non empty input slice up to the first occurrence of the pattern."] # [doc = ""] # [doc = " It doesn't consume the pattern. It will return `Err(Err::Error((_, ErrorKind::TakeUntil)))`"] # [doc = " if the pattern wasn't met."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::complete::take_until1;"] # [doc = ""] # [doc = " fn until_eof(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_until1(\"eof\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(until_eof(\"hello, worldeof\"), Ok((\"eof\", \"hello, world\")));"] # [doc = " assert_eq!(until_eof(\"hello, world\"), Err(Err::Error(Error::new(\"hello, world\", ErrorKind::TakeUntil))));"] # [doc = " assert_eq!(until_eof(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::TakeUntil))));"] # [doc = " assert_eq!(until_eof(\"1eof2eof\"), Ok((\"eof2eof\", \"1\")));"] # [doc = " assert_eq!(until_eof(\"eof\"), Err(Err::Error(Error::new(\"eof\", ErrorKind::TakeUntil))));"] # [doc = " ```"] pub fn take_until1 < T , I , Error : ParseError < I > > (tag : T) -> impl FnMut (I) -> IResult < I , I , Error > where I : Input + FindSubstring < T > , T : Input + Clone , { let mut parser = super :: take_until1 (tag) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Complete > > (i) }
};
}
