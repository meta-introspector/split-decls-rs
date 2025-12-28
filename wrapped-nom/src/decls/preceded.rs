macro_rules! deps {
    () => {
        ParseError!();
        Parser!();
        Error!();
        Preceded!();
        Needed!();
    };
}

macro_rules! preceded {
    () => {
        deps!();
        # [doc = " Matches an object from the first parser and discards it,"] # [doc = " then gets an object from the second parser."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `first` The opening parser."] # [doc = " * `second` The second parser to get object."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::sequence::preceded;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " let mut parser = preceded(tag(\"abc\"), tag(\"efg\"));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcefg\"), Ok((\"\", \"efg\")));"] # [doc = " assert_eq!(parser.parse(\"abcefghij\"), Ok((\"hij\", \"efg\")));"] # [doc = " assert_eq!(parser.parse(\"\"), Err(Err::Error((\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Tag))));"] # [doc = " ```"] pub fn preceded < I , O , E : ParseError < I > , F , G > (first : F , second : G ,) -> impl Parser < I , Output = O , Error = E > where F : Parser < I , Error = E > , G : Parser < I , Output = O , Error = E > , { Preceded { f : first , g : second , } }
    };
}

preceded!();