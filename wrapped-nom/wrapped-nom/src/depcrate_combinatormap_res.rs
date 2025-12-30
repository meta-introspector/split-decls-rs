// Generated macro for map_res (function)
macro_rules! Depcrate_combinatormap_res {
() => {
// Module: crate::combinator
// Provides: {"map_res"}
// Dependencies: {}
# [doc = " Applies a function returning a `Result` over the result of a parser."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::character::complete::digit1;"] # [doc = " use nom::combinator::map_res;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parse = map_res(digit1, |s: &str| s.parse::<u8>());"] # [doc = ""] # [doc = " // the parser will convert the result of digit1 to a number"] # [doc = " assert_eq!(parse.parse(\"123\"), Ok((\"\", 123)));"] # [doc = ""] # [doc = " // this will fail if digit1 fails"] # [doc = " assert_eq!(parse.parse(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Digit))));"] # [doc = ""] # [doc = " // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)"] # [doc = " assert_eq!(parse.parse(\"123456\"), Err(Err::Error((\"123456\", ErrorKind::MapRes))));"] # [doc = " # }"] # [doc = " ```"] pub fn map_res < I : Clone , O , E : ParseError < I > + FromExternalError < I , E2 > , E2 , F , G > (parser : F , f : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> Result < O , E2 > , { parser . map_res (f) }
};
}
