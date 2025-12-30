// Generated macro for tag_no_case (function)
macro_rules! Depcrate_bytestag_no_case {
() => {
// Module: crate::bytes
// Provides: {"tag_no_case"}
// Dependencies: {}
# [doc = " Recognizes a case insensitive pattern."] # [doc = ""] # [doc = " The input data will be compared to the tag combinator's argument and will return the part of"] # [doc = " the input that matches the argument with no regard to case."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::streaming::tag_no_case;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, &str> {"] # [doc = "   tag_no_case(\"hello\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"Hello, World!\"), Ok((\", World!\", \"Hello\")));"] # [doc = " assert_eq!(parser(\"hello, World!\"), Ok((\", World!\", \"hello\")));"] # [doc = " assert_eq!(parser(\"HeLlO, World!\"), Ok((\", World!\", \"HeLlO\")));"] # [doc = " assert_eq!(parser(\"Something\"), Err(Err::Error(Error::new(\"Something\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Incomplete(Needed::new(5))));"] # [doc = " ```"] pub fn tag_no_case < T , I , Error : ParseError < I > > (tag : T) -> impl Parser < I , Output = I , Error = Error > where I : Input + Compare < T > , T : Input + Clone , { TagNoCase { tag , e : PhantomData , } }
};
}
