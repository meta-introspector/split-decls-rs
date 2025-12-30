// Generated macro for verify (function)
macro_rules! Depcrate_combinatorverify {
() => {
// Module: crate::combinator
// Provides: {"verify"}
// Dependencies: {}
# [doc = " Returns the result of the child parser if it satisfies a verification function."] # [doc = ""] # [doc = " The verification function takes as argument a reference to the output of the"] # [doc = " parser."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::verify;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = verify(alpha1, |s: &str| s.len() == 4);"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcd\"), Ok((\"\", \"abcd\")));"] # [doc = " assert_eq!(parser.parse(\"abcde\"), Err(Err::Error((\"abcde\", ErrorKind::Verify))));"] # [doc = " assert_eq!(parser.parse(\"123abcd;\"),Err(Err::Error((\"123abcd;\", ErrorKind::Alpha))));"] # [doc = " # }"] # [doc = " ```"] pub fn verify < I : Clone , O2 , E : ParseError < I > , F , G > (first : F , second : G ,) -> impl Parser < I , Output = < F as Parser < I > > :: Output , Error = E > where F : Parser < I , Error = E > , G : Fn (& O2) -> bool , < F as Parser < I > > :: Output : Borrow < O2 > , O2 : ? Sized , { Verify { first , second , o2 : PhantomData , } }
};
}
