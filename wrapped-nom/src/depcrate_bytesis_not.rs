// Generated macro for is_not (function)
macro_rules! Depcrate_bytesis_not {
() => {
// Module: crate::bytes
// Provides: {"is_not"}
// Dependencies: {}
# [doc = " Parse till certain characters are met."] # [doc = ""] # [doc = " The parser will return the longest slice till one of the characters of the combinator's argument are met."] # [doc = ""] # [doc = " It doesn't consume the matched character."] # [doc = ""] # [doc = " It will return a `Err::Error((\"\", ErrorKind::IsNot))` if the pattern wasn't met."] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};"] # [doc = " use nom::bytes::complete::is_not;"] # [doc = ""] # [doc = " fn not_space(s: &str) -> IResult<&str, &str> {"] # [doc = "   is_not(\" \\t\\r\\n\")(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(not_space(\"Hello, World!\"), Ok((\" World!\", \"Hello,\")));"] # [doc = " assert_eq!(not_space(\"Sometimes\\t\"), Ok((\"\\t\", \"Sometimes\")));"] # [doc = " assert_eq!(not_space(\"Nospace\"), Ok((\"\", \"Nospace\")));"] # [doc = " assert_eq!(not_space(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::IsNot))));"] # [doc = " ```"] pub fn is_not < T , I , Error : ParseError < I > > (arr : T) -> impl Parser < I , Output = I , Error = Error > where I : Input , T : FindToken < < I as Input > :: Item > , { SplitPosition1 { e : ErrorKind :: IsNot , predicate : move | c | arr . find_token (c) , error : PhantomData , } }
};
}
