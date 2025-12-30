// Generated macro for cond (function)
macro_rules! Depcrate_combinatorcond {
() => {
// Module: crate::combinator
// Provides: {"cond"}
// Dependencies: {}
# [doc = " Calls the parser if the condition is met."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Parser};"] # [doc = " use nom::combinator::cond;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {"] # [doc = "   cond(b, alpha1).parse(i)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(true, \"abcd;\"), Ok((\";\", Some(\"abcd\"))));"] # [doc = " assert_eq!(parser(false, \"abcd;\"), Ok((\"abcd;\", None)));"] # [doc = " assert_eq!(parser(true, \"123;\"), Err(Err::Error(Error::new(\"123;\", ErrorKind::Alpha))));"] # [doc = " assert_eq!(parser(false, \"123;\"), Ok((\"123;\", None)));"] # [doc = " # }"] # [doc = " ```"] pub fn cond < I , E : ParseError < I > , F > (b : bool , f : F ,) -> impl Parser < I , Output = Option < < F as Parser < I > > :: Output > , Error = E > where F : Parser < I , Error = E > , { Cond { parser : if b { Some (f) } else { None } , } }
};
}
