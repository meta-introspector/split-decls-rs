// Generated macro for recognize (function)
macro_rules! Depcrate_combinatorrecognize {
() => {
// Module: crate::combinator
// Provides: {"recognize"}
// Dependencies: {}
# [doc = " If the child parser was successful, return the consumed input as produced value."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::recognize;"] # [doc = " use nom::character::complete::{char, alpha1};"] # [doc = " use nom::sequence::separated_pair;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = recognize(separated_pair(alpha1, char(','), alpha1));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcd,efgh\"), Ok((\"\", \"abcd,efgh\")));"] # [doc = " assert_eq!(parser.parse(\"abcd;\"),Err(Err::Error((\";\", ErrorKind::Char))));"] # [doc = " # }"] # [doc = " ```"] pub fn recognize < I : Clone + Offset + Input , E : ParseError < I > , F > (parser : F ,) -> impl Parser < I , Output = I , Error = E > where F : Parser < I , Error = E > , { Recognize { parser } }
};
}
