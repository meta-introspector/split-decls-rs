macro_rules! deps {
    () => {
        ErrorKind!();
        Parser!();
        IResult!();
        Error!();
        Needed!();
        ParseError!();
        Many1Count!();
        Input!();
    };
}

macro_rules! many1_count {
    () => {
        deps!();
        # [doc = " Runs the embedded parser, counting the results."] # [doc = ""] # [doc = " This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,"] # [doc = " see [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = ""] # [doc = " *Note*: If the parser passed to `many1` accepts empty inputs"] # [doc = " (like `alpha0` or `digit0`), `many1` will return an error,"] # [doc = " to prevent going into an infinite loop."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::many1_count;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, usize> {"] # [doc = "   many1_count(tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", 2)));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", 1)));"] # [doc = " assert_eq!(parser(\"123123\"), Err(Err::Error(Error::new(\"123123\", ErrorKind::Many1Count))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Many1Count))));"] # [doc = " ```"] pub fn many1_count < I , E , F > (parser : F) -> impl Parser < I , Output = usize , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , E : ParseError < I > , { Many1Count { parser } }
    };
}

many1_count!()