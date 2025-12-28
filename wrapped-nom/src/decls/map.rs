macro_rules! deps {
    () => {
        Error!();
        ParseError!();
        IResult!();
        Parser!();
    };
}

macro_rules! map {
    () => {
        deps!();
        # [doc = " Maps a function on the result of a parser."] # [doc = ""] # [doc = " ```rust"] # [doc = " use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::character::complete::digit1;"] # [doc = " use nom::combinator::map;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parser = map(digit1, |s: &str| s.len());"] # [doc = ""] # [doc = " // the parser will count how many characters were returned by digit1"] # [doc = " assert_eq!(parser.parse(\"123456\"), Ok((\"\", 6)));"] # [doc = ""] # [doc = " // this will fail if digit1 fails"] # [doc = " assert_eq!(parser.parse(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Digit))));"] # [doc = " # }"] # [doc = " ```"] pub fn map < I , O , E : ParseError < I > , F , G > (parser : F , f : G) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> O , { parser . map (f) }
    };
}

map!()