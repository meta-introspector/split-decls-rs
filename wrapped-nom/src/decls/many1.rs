macro_rules! deps {
    () => {
        Many1!();
        Parser!();
        IResult!();
        Error!();
        Input!();
        ErrorKind!();
        Needed!();
    };
}

macro_rules! many1 {
    () => {
        deps!();
        # [doc = " Runs the embedded parser, gathering the results in a `Vec`."] # [doc = ""] # [doc = " This stops on [`Err::Error`] if there is at least one result,  and returns the results that were accumulated. To instead chain an error up,"] # [doc = " see [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = ""] # [doc = " *Note*: If the parser passed to `many1` accepts empty inputs"] # [doc = " (like `alpha0` or `digit0`), `many1` will return an error,"] # [doc = " to prevent going into an infinite loop."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::many1;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   many1(tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"123123\"), Err(Err::Error(Error::new(\"123123\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Tag))));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn many1 < I , F > (parser : F ,) -> impl Parser < I , Output = Vec < < F as Parser < I > > :: Output > , Error = < F as Parser < I > > :: Error > where I : Clone + Input , F : Parser < I > , { Many1 { parser } }
    };
}

many1!();