macro_rules! deps {
    () => {
        ToUsize!();
        Error!();
        ParseError!();
        Input!();
        IResult!();
        Parser!();
        Needed!();
    };
}

macro_rules! length_data {
    () => {
        deps!();
        # [doc = " Gets a number from the parser and returns a"] # [doc = " subslice of the input of that size."] # [doc = " If the parser returns `Incomplete`,"] # [doc = " `length_data` will return an error."] # [doc = " # Arguments"] # [doc = " * `f` The parser to apply."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};"] # [doc = " use nom::number::complete::be_u16;"] # [doc = " use nom::multi::length_data;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   length_data(be_u16).parse(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(b\"\\x00\\x03abcefg\"), Ok((&b\"efg\"[..], &b\"abc\"[..])));"] # [doc = " assert_eq!(parser(b\"\\x00\\x03a\"), Err(Err::Incomplete(Needed::new(2))));"] # [doc = " ```"] pub fn length_data < I , E , F > (f : F) -> impl Parser < I , Output = I , Error = E > where I : Input , < F as Parser < I > > :: Output : ToUsize , F : Parser < I , Error = E > , E : ParseError < I > , { f . flat_map (| size | take (size)) }
    };
}

length_data!();