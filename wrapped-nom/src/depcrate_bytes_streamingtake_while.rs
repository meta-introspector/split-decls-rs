// Generated macro for take_while (function)
macro_rules! Depcrate_bytes_streamingtake_while {
() => {
// Module: crate::bytes::streaming
// Provides: {"take_while"}
// Dependencies: {}
# [doc = " Returns the longest input slice (if any) that matches the predicate."] # [doc = ""] # [doc = " The parser will return the longest slice that matches the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the pattern reaches the end of the input."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_while;"] # [doc = " use nom::AsChar;"] # [doc = ""] # [doc = " fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   take_while(AsChar::is_alpha)(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(alpha(b\"latin123\"), Ok((&b\"123\"[..], &b\"latin\"[..])));"] # [doc = " assert_eq!(alpha(b\"12345\"), Ok((&b\"12345\"[..], &b\"\"[..])));"] # [doc = " assert_eq!(alpha(b\"latin\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(alpha(b\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn take_while < F , I , Error : ParseError < I > > (cond : F) -> impl FnMut (I) -> IResult < I , I , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { let mut parser = super :: take_while (cond) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
