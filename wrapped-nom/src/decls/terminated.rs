macro_rules! deps {
    () => {
        Needed!();
        Parser!();
        ParseError!();
        Error!();
        Terminated!();
    };
}

macro_rules! terminated {
    () => {
        deps!();
        # [doc = " Gets an object from the first parser,"] # [doc = " then matches an object from the second parser and discards it."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `first` The first parser to apply."] # [doc = " * `second` The second parser to match an object."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::sequence::terminated;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " let mut parser = terminated(tag(\"abc\"), tag(\"efg\"));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcefg\"), Ok((\"\", \"abc\")));"] # [doc = " assert_eq!(parser.parse(\"abcefghij\"), Ok((\"hij\", \"abc\")));"] # [doc = " assert_eq!(parser.parse(\"\"), Err(Err::Error((\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Tag))));"] # [doc = " ```"] pub fn terminated < I , O , E : ParseError < I > , F , G > (first : F , second : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Output = O , Error = E > , G : Parser < I , Error = E > , { Terminated { f : first , g : second , } }
    };
}

terminated!();