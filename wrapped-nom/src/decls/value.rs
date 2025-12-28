macro_rules! deps {
    () => {
        Error!();
        IResult!();
        Parser!();
        ParseError!();
    };
}

macro_rules! value {
    () => {
        deps!();
        # [doc = " Returns the provided value if the child parser succeeds."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::value;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = value(1234, alpha1);"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcd\"), Ok((\"\", 1234)));"] # [doc = " assert_eq!(parser.parse(\"123abcd;\"), Err(Err::Error((\"123abcd;\", ErrorKind::Alpha))));"] # [doc = " # }"] # [doc = " ```"] pub fn value < I , O1 : Clone , E : ParseError < I > , F > (val : O1 , parser : F ,) -> impl Parser < I , Output = O1 , Error = E > where F : Parser < I , Error = E > , { parser . map (move | _ | val . clone ()) }
    };
}

value!()