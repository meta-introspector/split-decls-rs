macro_rules! deps {
    () => {
        Parser!();
        IResult!();
        Error!();
        ParseError!();
    };
}

macro_rules! map_parser {
    () => {
        deps!();
        # [doc = " Applies a parser over the result of another one."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::character::complete::digit1;"] # [doc = " use nom::bytes::complete::take;"] # [doc = " use nom::combinator::map_parser;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parse = map_parser(take(5u8), digit1);"] # [doc = ""] # [doc = " assert_eq!(parse.parse(\"12345\"), Ok((\"\", \"12345\")));"] # [doc = " assert_eq!(parse.parse(\"123ab\"), Ok((\"\", \"123\")));"] # [doc = " assert_eq!(parse.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Eof))));"] # [doc = " # }"] # [doc = " ```"] pub fn map_parser < I , O , E : ParseError < I > , F , G > (parser : F , applied_parser : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : Parser < < F as Parser < I > > :: Output , Output = O , Error = E > , { parser . and_then (applied_parser) }
    };
}

map_parser!()