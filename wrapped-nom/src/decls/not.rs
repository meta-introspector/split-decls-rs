macro_rules! deps {
    () => {
        IResult!();
        ParseError!();
        Not!();
        Error!();
        Parser!();
    };
}

macro_rules! not {
    () => {
        deps!();
        # [doc = " Succeeds if the child parser returns an error."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::not;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = not(alpha1);"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"123\"), Ok((\"123\", ())));"] # [doc = " assert_eq!(parser.parse(\"abcd\"), Err(Err::Error((\"abcd\", ErrorKind::Not))));"] # [doc = " # }"] # [doc = " ```"] pub fn not < I : Clone , E : ParseError < I > , F > (parser : F) -> impl Parser < I , Output = () , Error = E > where F : Parser < I , Error = E > , { Not { parser } }
    };
}

not!();