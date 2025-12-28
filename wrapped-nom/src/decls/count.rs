macro_rules! deps {
    () => {
        Parser!();
        Count!();
        ErrorKind!();
        IResult!();
        Needed!();
        Error!();
    };
}

macro_rules! count {
    () => {
        deps!();
        # [doc = " Runs the embedded parser `count` times, gathering the results in a `Vec`"] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = " * `count` How often to apply the parser."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::count;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, Vec<&str>> {"] # [doc = "   count(tag(\"abc\"), 2).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", vec![\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Err(Err::Error(Error::new(\"123\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"123123\"), Err(Err::Error(Error::new(\"123123\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"abcabcabc\"), Ok((\"abc\", vec![\"abc\", \"abc\"])));"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn count < I , F > (parser : F , count : usize ,) -> impl Parser < I , Output = Vec < < F as Parser < I > > :: Output > , Error = < F as Parser < I > > :: Error > where I : Clone , F : Parser < I > , { Count { parser , count } }
    };
}

count!()