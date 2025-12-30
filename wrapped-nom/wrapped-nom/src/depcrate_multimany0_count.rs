// Generated macro for many0_count (function)
macro_rules! Depcrate_multimany0_count {
() => {
// Module: crate::multi
// Provides: {"many0_count"}
// Dependencies: {}
# [doc = " Repeats the embedded parser, counting the results"] # [doc = ""] # [doc = " This stops on [`Err::Error`]. To instead chain an error up, see"] # [doc = " [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = ""] # [doc = " *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will"] # [doc = " return an error, to prevent going into an infinite loop"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::multi::many0_count;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, usize> {"] # [doc = "   many0_count(tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", 2)));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", 1)));"] # [doc = " assert_eq!(parser(\"123123\"), Ok((\"123123\", 0)));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", 0)));"] # [doc = " ```"] pub fn many0_count < I , E , F > (parser : F) -> impl Parser < I , Output = usize , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , E : ParseError < I > , { Many0Count { parser } }
};
}
