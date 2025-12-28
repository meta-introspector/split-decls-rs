macro_rules! deps {
    () => {
        IResult!();
        ParseError!();
        Parser!();
        Error!();
        Cut!();
    };
}

macro_rules! cut {
    () => {
        deps!();
        # [doc = " Transforms an [`Err::Error`] (recoverable) to [`Err::Failure`] (unrecoverable)"] # [doc = ""] # [doc = " This commits the parse result, preventing alternative branch paths like with"] # [doc = " [`nom::branch::alt`][crate::branch::alt]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Without `cut`:"] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " # use nom::character::complete::{one_of, digit1};"] # [doc = " # use nom::combinator::rest;"] # [doc = " # use nom::branch::alt;"] # [doc = " # use nom::sequence::preceded;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "   alt(("] # [doc = "     preceded(one_of(\"+-\"), digit1),"] # [doc = "     rest"] # [doc = "   )).parse(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"+10 ab\"), Ok((\" ab\", \"10\")));"] # [doc = " assert_eq!(parser(\"ab\"), Ok((\"\", \"ab\")));"] # [doc = " assert_eq!(parser(\"+\"), Ok((\"\", \"+\")));"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " With `cut`:"] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser, error::Error};"] # [doc = " # use nom::character::complete::{one_of, digit1};"] # [doc = " # use nom::combinator::rest;"] # [doc = " # use nom::branch::alt;"] # [doc = " # use nom::sequence::preceded;"] # [doc = " use nom::combinator::cut;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "   alt(("] # [doc = "     preceded(one_of(\"+-\"), cut(digit1)),"] # [doc = "     rest"] # [doc = "   )).parse(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"+10 ab\"), Ok((\" ab\", \"10\")));"] # [doc = " assert_eq!(parser(\"ab\"), Ok((\"\", \"ab\")));"] # [doc = " assert_eq!(parser(\"+\"), Err(Err::Failure(Error { input: \"\", code: ErrorKind::Digit })));"] # [doc = " # }"] # [doc = " ```"] pub fn cut < I , E : ParseError < I > , F > (parser : F ,) -> impl Parser < I , Output = < F as Parser < I > > :: Output , Error = E > where F : Parser < I , Error = E > , { Cut { parser } }
    };
}

cut!();