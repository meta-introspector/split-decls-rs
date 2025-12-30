// Generated macro for all_consuming (function)
macro_rules! Depcrate_combinatorall_consuming {
() => {
// Module: crate::combinator
// Provides: {"all_consuming"}
// Dependencies: {}
# [doc = " Succeeds if all the input has been consumed by its child parser."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::all_consuming;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = all_consuming(alpha1);"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcd\"), Ok((\"\", \"abcd\")));"] # [doc = " assert_eq!(parser.parse(\"abcd;\"),Err(Err::Error((\";\", ErrorKind::Eof))));"] # [doc = " assert_eq!(parser.parse(\"123abcd;\"),Err(Err::Error((\"123abcd;\", ErrorKind::Alpha))));"] # [doc = " # }"] # [doc = " ```"] pub fn all_consuming < I , E : ParseError < I > , F > (parser : F ,) -> impl Parser < I , Output = < F as Parser < I > > :: Output , Error = E > where I : Input , F : Parser < I , Error = E > , { AllConsuming { parser } }
};
}
