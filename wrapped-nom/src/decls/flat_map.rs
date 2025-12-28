macro_rules! deps {
    () => {
        Parser!();
        Error!();
        ParseError!();
        IResult!();
    };
}

macro_rules! flat_map {
    () => {
        deps!();
        # [doc = " Creates a new parser from the output of the first parser, then apply that parser over the rest of the input."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::bytes::complete::take;"] # [doc = " use nom::number::complete::u8;"] # [doc = " use nom::combinator::flat_map;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut parse = flat_map(u8, take);"] # [doc = ""] # [doc = " assert_eq!(parse.parse(&[2, 0, 1, 2][..]), Ok((&[2][..], &[0, 1][..])));"] # [doc = " assert_eq!(parse.parse(&[4, 0, 1, 2][..]), Err(Err::Error((&[0, 1, 2][..], ErrorKind::Eof))));"] # [doc = " # }"] # [doc = " ```"] pub fn flat_map < I , O , E : ParseError < I > , F , G , H > (parser : F , applied_parser : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> H , H : Parser < I , Output = O , Error = E > , { parser . flat_map (applied_parser) }
    };
}

flat_map!();