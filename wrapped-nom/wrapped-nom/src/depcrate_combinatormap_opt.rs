// Generated macro for map_opt (function)
macro_rules! Depcrate_combinatormap_opt {
() => {
// Module: crate::combinator
// Provides: {"map_opt"}
// Dependencies: {}
# [doc = " Applies a function returning an `Option` over the result of a parser."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::character::complete::digit1;"] # [doc = " use nom::combinator::map_opt;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parse = map_opt(digit1, |s: &str| s.parse::<u8>().ok());"] # [doc = ""] # [doc = " // the parser will convert the result of digit1 to a number"] # [doc = " assert_eq!(parse.parse(\"123\"), Ok((\"\", 123)));"] # [doc = ""] # [doc = " // this will fail if digit1 fails"] # [doc = " assert_eq!(parse.parse(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Digit))));"] # [doc = ""] # [doc = " // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)"] # [doc = " assert_eq!(parse.parse(\"123456\"), Err(Err::Error((\"123456\", ErrorKind::MapOpt))));"] # [doc = " # }"] # [doc = " ```"] pub fn map_opt < I : Clone , O , E : ParseError < I > , F , G > (parser : F , f : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> Option < O > , { parser . map_opt (f) }
};
}
