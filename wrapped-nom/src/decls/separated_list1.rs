macro_rules! deps {
    () => {
        ErrorKind!();
        Parser!();
        Needed!();
        ParseError!();
        Error!();
        Input!();
        SeparatedList1!();
        IResult!();
    };
}

macro_rules! separated_list1 {
    () => {
        deps!();
        # [doc = " Alternates between two parsers to produce a list of elements until [`Err::Error`]."] # [doc = ""] # [doc = " Fails if the element parser does not produce at least one element."] # [doc = ""] # [doc = " This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see"] # [doc = " [`cut`][crate::combinator::cut]."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `sep` Parses the separator between list elements."] # [doc = " * `f` Parses the elements of the list."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::separated_list1;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   separated_list1(tag(\"|\"), tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abc|abc|abc\"), Ok((\"\", vec![\"abc\", \"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123abc\"), Ok((\"123abc\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"abc|def\"), Ok((\"|def\", vec![\"abc\"])));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"def|abc\"), Err(Err::Error(Error::new(\"def|abc\", ErrorKind::Tag))));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn separated_list1 < I , E , F , G > (separator : G , parser : F ,) -> impl Parser < I , Output = Vec < < F as Parser < I > > :: Output > , Error = E > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { SeparatedList1 { parser , separator } }
    };
}

separated_list1!();