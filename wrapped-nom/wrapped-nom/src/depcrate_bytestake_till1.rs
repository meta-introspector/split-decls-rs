// Generated macro for take_till1 (function)
macro_rules! Depcrate_bytestake_till1 {
() => {
// Module: crate::bytes
// Provides: {"take_till1"}
// Dependencies: {}
# [doc = " Returns the longest (at least 1) input slice till a predicate is met."] # [doc = ""] # [doc = " The parser will return the longest slice till the given predicate *(a function that"] # [doc = " takes the input and returns a bool)*."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the match reaches the"] # [doc = " end of input or if there was not match."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take_till1;"] # [doc = ""] # [doc = " fn till_colon(s: &str) -> IResult<&str, &str> {"] # [doc = "   take_till1(|c| c == ':')(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(till_colon(\"latin:123\"), Ok((\":123\", \"latin\")));"] # [doc = " assert_eq!(till_colon(\":empty matched\"), Err(Err::Error(Error::new(\":empty matched\", ErrorKind::TakeTill1))));"] # [doc = " assert_eq!(till_colon(\"12345\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " assert_eq!(till_colon(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] # [allow (clippy :: redundant_closure)] pub fn take_till1 < F , I , Error : ParseError < I > > (cond : F) -> impl Parser < I , Output = I , Error = Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { SplitPosition1 { e : ErrorKind :: TakeTill1 , predicate : cond , error : PhantomData , } }
};
}
