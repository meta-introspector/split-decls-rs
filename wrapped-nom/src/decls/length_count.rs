macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        LengthCount!();
        ParseError!();
        ToUsize!();
        Needed!();
        Parser!();
        IResult!();
    };
}

macro_rules! length_count {
    () => {
        deps!();
        # [doc = " Gets a number from the first parser,"] # [doc = " then applies the second parser that many times."] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply to obtain the count."] # [doc = " * `g` The parser to apply repeatedly."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::number::complete::u8;"] # [doc = " use nom::multi::length_count;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = " use nom::combinator::map;"] # [doc = ""] # [doc = " fn parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {"] # [doc = "   length_count(map(u8, |i| {"] # [doc = "      println!(\"got number: {}\", i);"] # [doc = "      i"] # [doc = "   }), tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(&b\"\\x02abcabcabc\"[..]), Ok(((&b\"abc\"[..], vec![&b\"abc\"[..], &b\"abc\"[..]]))));"] # [doc = " assert_eq!(parser(b\"\\x03123123123\"), Err(Err::Error(Error::new(&b\"123123123\"[..], ErrorKind::Tag))));"] # [doc = " ```"] # [cfg (feature = "alloc")] pub fn length_count < I , E , F , G > (f : F , g : G ,) -> impl Parser < I , Output = Vec < < G as Parser < I > > :: Output > , Error = E > where I : Clone , < F as Parser < I > > :: Output : ToUsize , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { LengthCount { length : f , parser : g , e : PhantomData , } }
    };
}

length_count!()