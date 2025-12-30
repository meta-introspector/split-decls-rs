// Generated macro for complete (function)
macro_rules! Depcrate_combinatorcomplete {
() => {
// Module: crate::combinator
// Provides: {"complete"}
// Dependencies: {}
# [doc = " Transforms Incomplete into `Error`."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::bytes::streaming::take;"] # [doc = " use nom::combinator::complete;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = complete(take(5u8));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcdefg\"), Ok((\"fg\", \"abcde\")));"] # [doc = " assert_eq!(parser.parse(\"abcd\"), Err(Err::Error((\"abcd\", ErrorKind::Complete))));"] # [doc = " # }"] # [doc = " ```"] pub fn complete < I : Clone , O , E : ParseError < I > , F > (parser : F ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Output = O , Error = E > , { MakeComplete { parser } }
};
}
