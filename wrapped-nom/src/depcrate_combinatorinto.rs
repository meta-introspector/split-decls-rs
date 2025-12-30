// Generated macro for into (function)
macro_rules! Depcrate_combinatorinto {
() => {
// Module: crate::combinator
// Provides: {"into"}
// Dependencies: {}
# [doc = " automatically converts the child parser's result to another type"] # [doc = ""] # [doc = " it will be able to convert the output value and the error value"] # [doc = " as long as the `Into` implementations are available"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{IResult, Parser};"] # [doc = " use nom::combinator::into;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser1(i: &str) -> IResult<&str, &str> {"] # [doc = "   alpha1(i)"] # [doc = " }"] # [doc = ""] # [doc = " let mut parser2 = into(parser1);"] # [doc = ""] # [doc = " // the parser converts the &str output of the child parser into a Vec<u8>"] # [doc = " let bytes: IResult<&str, Vec<u8>> = parser2.parse(\"abcd\");"] # [doc = " assert_eq!(bytes, Ok((\"\", vec![97, 98, 99, 100])));"] # [doc = " # }"] # [doc = " ```"] pub fn into < I , O1 , O2 , E1 , E2 , F > (parser : F) -> impl Parser < I , Output = O2 , Error = E2 > where O2 : From < O1 > , E2 : From < E1 > , E1 : ParseError < I > , E2 : ParseError < I > , F : Parser < I , Output = O1 , Error = E1 > , { parser . into :: < O2 , E2 > () }
};
}
