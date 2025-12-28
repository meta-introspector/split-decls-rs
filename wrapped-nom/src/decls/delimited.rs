macro_rules! deps {
    () => {
        ParseError!();
        Error!();
        Parser!();
        Needed!();
    };
}

macro_rules! delimited {
    () => {
        deps!();
        # [doc = " Matches an object from the first parser and discards it,"] # [doc = " then gets an object from the second parser,"] # [doc = " and finally matches an object from the third parser and discards it."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `first` The first parser to apply and discard."] # [doc = " * `second` The second parser to apply."] # [doc = " * `third` The third parser to apply and discard."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::sequence::delimited;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " let mut parser = delimited(tag(\"(\"), tag(\"abc\"), tag(\")\"));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"(abc)\"), Ok((\"\", \"abc\")));"] # [doc = " assert_eq!(parser.parse(\"(abc)def\"), Ok((\"def\", \"abc\")));"] # [doc = " assert_eq!(parser.parse(\"\"), Err(Err::Error((\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Tag))));"] # [doc = " ```"] pub fn delimited < I , O , E : ParseError < I > , F , G , H > (first : F , second : G , third : H ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : Parser < I , Output = O , Error = E > , H : Parser < I , Error = E > , { preceded (first , terminated (second , third)) }
    };
}

delimited!();