macro_rules! deps {
    () => {
        IResult!();
        ParseError!();
        Fill!();
        Needed!();
        Error!();
        Parser!();
        ErrorKind!();
    };
}

macro_rules! fill {
    () => {
        deps!();
        # [doc = " Runs the embedded parser repeatedly, filling the given slice with results."] # [doc = ""] # [doc = " This parser fails if the input runs out before the given slice is full."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = " * `buf` The slice to fill"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::multi::fill;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &str) -> IResult<&str, [&str; 2]> {"] # [doc = "   let mut buf = [\"\", \"\"];"] # [doc = "   let (rest, ()) = fill(tag(\"abc\"), &mut buf).parse(s)?;"] # [doc = "   Ok((rest, buf))"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abcabc\"), Ok((\"\", [\"abc\", \"abc\"])));"] # [doc = " assert_eq!(parser(\"abc123\"), Err(Err::Error(Error::new(\"123\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"123123\"), Err(Err::Error(Error::new(\"123123\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(\"abcabcabc\"), Ok((\"abc\", [\"abc\", \"abc\"])));"] # [doc = " ```"] pub fn fill < 'a , I , E , F > (parser : F , buf : & 'a mut [< F as Parser < I > > :: Output] ,) -> impl Parser < I , Output = () , Error = E > + 'a where I : Clone , F : Parser < I , Error = E > + 'a , E : ParseError < I > , { Fill { parser , buf } }
    };
}

fill!()