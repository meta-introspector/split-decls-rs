// Generated macro for tag (function)
macro_rules! Depcrate_bytes_streamingtag {
() => {
// Module: crate::bytes::streaming
// Provides: {"tag"}
// Dependencies: {}
# [doc = " Recognizes a pattern."] # [doc = ""] # [doc = " The input data will be compared to the tag combinator's argument and will return the part of"] # [doc = " the input that matches the argument."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::streaming::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, &str> {"] # [doc = "   tag(\"Hello\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"Hello, World!\"), Ok((\", World!\", \"Hello\")));"] # [doc = " assert_eq!(parser(\"Something\"), Err(Err::Error(Error::new(\"Something\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"S\"), Err(Err::Error(Error::new(\"S\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"H\"), Err(Err::Incomplete(Needed::new(4))));"] # [doc = " ```"] pub fn tag < T , I , Error : ParseError < I > > (tag : T) -> impl Fn (I) -> IResult < I , I , Error > where I : Input + Compare < T > , T : Input + Clone , { move | i : I | { let mut parser = super :: Tag { tag : tag . clone () , e : PhantomData , } ; parser . process :: < OutputM < Emit , Emit , Streaming > > (i) } }
};
}
