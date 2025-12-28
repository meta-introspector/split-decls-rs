macro_rules! deps {
    () => {
        Choice!();
        Parser!();
        Needed!();
        IResult!();
    };
}

macro_rules! alt {
    () => {
        deps!();
        # [doc = " Tests a list of parsers one by one until one succeeds."] # [doc = ""] # [doc = " It takes as argument either a tuple or an array of parsers. If using a"] # [doc = " tuple, there is a maximum of 21 parsers. If you need more, it is possible to"] # [doc = " use an array."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::error_position;"] # [doc = " # use nom::{Err,error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::character::complete::{alpha1, digit1};"] # [doc = " use nom::branch::alt;"] # [doc = " # fn main() {"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "   alt((alpha1, digit1)).parse(input)"] # [doc = " };"] # [doc = ""] # [doc = " // the first parser, alpha1, recognizes the input"] # [doc = " assert_eq!(parser(\"abc\"), Ok((\"\", \"abc\")));"] # [doc = ""] # [doc = " // the first parser returns an error, so alt tries the second one"] # [doc = " assert_eq!(parser(\"123456\"), Ok((\"\", \"123456\")));"] # [doc = ""] # [doc = " // both parsers failed, and with the default error type, alt will return the last error"] # [doc = " assert_eq!(parser(\" \"), Err(Err::Error(error_position!(\" \", ErrorKind::Digit))));"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " With a custom error type, it is possible to have alt return the error of the parser"] # [doc = " that went the farthest in the input data"] pub fn alt < List > (l : List) -> Choice < List > { Choice { parser : l } }
    };
}

alt!()