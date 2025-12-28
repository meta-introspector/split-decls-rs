macro_rules! deps {
    () => {
        IResult!();
        Error!();
        Parser!();
        Input!();
        Needed!();
        Many0!();
    };
}

macro_rules! many0 {
    () => {
        deps!();
        # [doc = " Repeats the embedded parser, gathering the results in a `Vec`."] # [doc = ""] # [doc = " This stops on [`Err::Error`] and returns the results that were accumulated. To instead chain an error up, see"] # [doc = " [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = ""] # [doc = " *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will"] # [doc = " return an error, to prevent going into an infinite loop"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::multi::many0;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   many0(tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Ok((\"123\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"123123\"), Ok((\"123123\", vec![])));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", vec![])));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn many0 < I , F > (f : F ,) -> impl Parser < I , Output = Vec < < F as Parser < I > > :: Output > , Error = < F as Parser < I > > :: Error > where I : Clone + Input , F : Parser < I > , { Many0 { parser : f } }
    };
}

many0!()