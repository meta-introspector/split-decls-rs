macro_rules! deps {
    () => {
        ParseError!();
        Input!();
        IResult!();
        Needed!();
        Error!();
        SeparatedList0!();
        Parser!();
    };
}

macro_rules! separated_list0 {
    () => {
        deps!();
        # [doc = " Alternates between two parsers to produce a list of elements."] # [doc = ""] # [doc = " This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see"] # [doc = " [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `sep` Parses the separator between list elements."] # [doc = " * `f` Parses the elements of the list."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::multi::separated_list0;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   separated_list0(tag(\"|\"), tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abc|abc|abc\"), Ok((\"\", vec![\"abc\", \"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123abc\"), Ok((\"123abc\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"abc|def\"), Ok((\"|def\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", vec![])));"] # [doc = " assert_eq!(parser(\"def|abc\"), Ok((\"def|abc\", vec![])));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn separated_list0 < I , E , F , G > (sep : G , f : F ,) -> impl Parser < I , Output = Vec < < F as Parser < I > > :: Output > , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { SeparatedList0 { parser : f , separator : sep , } }
    };
}

separated_list0!();