macro_rules! deps {
    () => {
        IResult!();
        ErrorKind!();
        LengthValue!();
        Error!();
        Parser!();
        Input!();
        ToUsize!();
        ParseError!();
        Needed!();
    };
}

macro_rules! length_value {
    () => {
        deps!();
        # [doc = " Gets a number from the first parser,"] # [doc = " takes a subslice of the input of that size,"] # [doc = " then applies the second parser on that subslice."] # [doc = " If the second parser returns `Incomplete`,"] # [doc = " `length_value` will return an error."] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = " * `g` The parser to apply on the subslice."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};"] # [doc = " use nom::number::complete::be_u16;"] # [doc = " use nom::multi::length_value;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   length_value(be_u16, tag(\"abc\")).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(b\"\\x00\\x03abcefg\"), Ok((&b\"efg\"[..], &b\"abc\"[..])));"] # [doc = " assert_eq!(parser(b\"\\x00\\x03123123\"), Err(Err::Error(Error::new(&b\"123\"[..], ErrorKind::Tag))));"] # [doc = " assert_eq!(parser(b\"\\x00\\x03a\"), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] pub fn length_value < I , E , F , G > (f : F , g : G ,) -> impl Parser < I , Output = < G as Parser < I > > :: Output , Error = E > where I : Clone + Input , < F as Parser < I > > :: Output : ToUsize , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { LengthValue { length : f , parser : g , e : PhantomData , } }
    };
}

length_value!()