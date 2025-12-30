// Generated macro for is_a (function)
macro_rules! Depcrate_bytes_streamingis_a {
() => {
// Module: crate::bytes::streaming
// Provides: {"is_a"}
// Dependencies: {}
# [doc = " Returns the longest input slice (at least 1) that matches the pattern."] # [doc = ""] # [doc = " The parser will return the longest slice consisting of the characters in provided in the"] # [doc = " combinator's argument."] # [doc = ""] # [doc = " # Streaming specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the pattern wasn't met"] # [doc = " or if the pattern reaches the end of the input."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::streaming::is_a;"] # [doc = ""] # [doc = " fn hex(s: &str) -> IResult<&str, &str> {"] # [doc = "   is_a(\"1234567890ABCDEF\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(hex(\"123 and voila\"), Ok((\" and voila\", \"123\")));"] # [doc = " assert_eq!(hex(\"DEADBEEF and others\"), Ok((\" and others\", \"DEADBEEF\")));"] # [doc = " assert_eq!(hex(\"BADBABEsomething\"), Ok((\"something\", \"BADBABE\")));"] # [doc = " assert_eq!(hex(\"D15EA5E\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(hex(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn is_a < T , I , Error : ParseError < I > > (arr : T) -> impl FnMut (I) -> IResult < I , I , Error > where I : Input , T : FindToken < < I as Input > :: Item > , { let mut parser = super :: is_a (arr) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
