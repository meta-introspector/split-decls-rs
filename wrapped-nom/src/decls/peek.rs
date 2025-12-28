macro_rules! deps {
    () => {
        Error!();
        Peek!();
        IResult!();
        Parser!();
    };
}

macro_rules! peek {
    () => {
        deps!();
        # [doc = " Tries to apply its parser without consuming the input."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::peek;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = peek(alpha1);"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcd;\"), Ok((\"abcd;\", \"abcd\")));"] # [doc = " assert_eq!(parser.parse(\"123;\"), Err(Err::Error((\"123;\", ErrorKind::Alpha))));"] # [doc = " # }"] # [doc = " ```"] pub fn peek < I : Clone , F > (parser : F ,) -> impl Parser < I , Output = < F as Parser < I > > :: Output , Error = < F as Parser < I > > :: Error > where F : Parser < I > , { Peek { parser } }
    };
}

peek!()