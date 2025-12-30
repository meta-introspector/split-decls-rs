// Generated macro for take_till (function)
macro_rules! Depcrate_bytes_streamingtake_till {
() => {
// Module: crate::bytes::streaming
// Provides: {"take_till"}
// Dependencies: {}
# [doc = " Returns the longest input slice (if any) till a predicate is met."] # [doc = ""] # [doc = " The parser will return the longest slice till the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the match reaches the"] # [doc = " end of input or if there was not match."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_till;"] # [doc = ""] # [doc = " fn till_colon(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_till(|c| c == ':')(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(till_colon(\"latin:123\"), Ok((\":123\", \"latin\")));"] # [doc = " assert_eq!(till_colon(\":empty matched\"), Ok((\":empty matched\", \"\"))); //allowed"] # [doc = " assert_eq!(till_colon(\"12345\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(till_colon(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [allow (clippy :: redundant_closure)] pub fn take_till < F , I , Error : ParseError < I > > (cond : F) -> impl FnMut (I) -> IResult < I , I , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { let mut parser = super :: take_till (cond) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
