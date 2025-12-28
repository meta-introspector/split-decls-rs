macro_rules! deps {
    () => {
        Parser!();
        Offset!();
        Consumed!();
        ParseError!();
        IResult!();
        Input!();
        Error!();
    };
}

macro_rules! consumed {
    () => {
        deps!();
        # [doc = " if the child parser was successful, return the consumed input with the output"] # [doc = " as a tuple. Functions similarly to [recognize](fn.recognize.html) except it"] # [doc = " returns the parser output as well."] # [doc = ""] # [doc = " This can be useful especially in cases where the output is not the same type"] # [doc = " as the input, or the input is a user defined type."] # [doc = ""] # [doc = " Returned tuple is of the format `(consumed input, produced output)`."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err,error::ErrorKind, IResult, Parser};"] # [doc = " use nom::combinator::{consumed, value, recognize, map};"] # [doc = " use nom::character::complete::{char, alpha1};"] # [doc = " use nom::bytes::complete::tag;"] # [doc = " use nom::sequence::separated_pair;"] # [doc = ""] # [doc = " fn inner_parser(input: &str) -> IResult<&str, bool> {"] # [doc = "     value(true, tag(\"1234\")).parse(input)"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut consumed_parser = consumed(value(true, separated_pair(alpha1, char(','), alpha1)));"] # [doc = ""] # [doc = " assert_eq!(consumed_parser.parse(\"abcd,efgh1\"), Ok((\"1\", (\"abcd,efgh\", true))));"] # [doc = " assert_eq!(consumed_parser.parse(\"abcd;\"),Err(Err::Error((\";\", ErrorKind::Char))));"] # [doc = ""] # [doc = ""] # [doc = " // the first output (representing the consumed input)"] # [doc = " // should be the same as that of the `recognize` parser."] # [doc = " let mut recognize_parser = recognize(inner_parser);"] # [doc = " let mut consumed_parser = map(consumed(inner_parser), |(consumed, output)| consumed);"] # [doc = ""] # [doc = " assert_eq!(recognize_parser.parse(\"1234\"), consumed_parser.parse(\"1234\"));"] # [doc = " assert_eq!(recognize_parser.parse(\"abcd\"), consumed_parser.parse(\"abcd\"));"] # [doc = " # }"] # [doc = " ```"] pub fn consumed < I , F , E > (parser : F ,) -> impl Parser < I , Output = (I , < F as Parser < I > > :: Output) , Error = E > where I : Clone + Offset + Input , E : ParseError < I > , F : Parser < I , Error = E > , { Consumed { parser } }
    };
}

consumed!()