macro_rules! deps {
    () => {
        Parser!();
        Error!();
        ParseError!();
        Opt!();
        IResult!();
    };
}

macro_rules! opt {
    () => {
        deps!();
        # [doc = " Optional parser, will return `None` on [`Err::Error`]."] # [doc = ""] # [doc = " To chain an error up, see [`cut`]."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::opt;"] # [doc = " use nom::character::complete::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser(i: &str) -> IResult<&str, Option<&str>> {"] # [doc = "   opt(alpha1).parse(i)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcd;\"), Ok((\";\", Some(\"abcd\"))));"] # [doc = " assert_eq!(parser(\"123;\"), Ok((\"123;\", None)));"] # [doc = " # }"] # [doc = " ```"] pub fn opt < I : Clone , E : ParseError < I > , F > (f : F ,) -> impl Parser < I , Output = Option < < F as Parser < I > > :: Output > , Error = E > where F : Parser < I , Error = E > , { Opt { parser : f } }
    };
}

opt!();