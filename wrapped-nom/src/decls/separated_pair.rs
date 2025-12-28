macro_rules! deps {
    () => {
        Error!();
        Needed!();
        ParseError!();
        Parser!();
    };
}

macro_rules! separated_pair {
    () => {
        deps!();
        # [doc = " Gets an object from the first parser,"] # [doc = " then matches an object from the sep_parser and discards it,"] # [doc = " then gets another object from the second parser."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `first` The first parser to apply."] # [doc = " * `sep` The separator parser to apply."] # [doc = " * `second` The second parser to apply."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::sequence::separated_pair;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = ""] # [doc = " let mut parser = separated_pair(tag(\"abc\"), tag(\"|\"), tag(\"efg\"));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abc|efg\"), Ok((\"\", (\"abc\", \"efg\"))));"] # [doc = " assert_eq!(parser.parse(\"abc|efghij\"), Ok((\"hij\", (\"abc\", \"efg\"))));"] # [doc = " assert_eq!(parser.parse(\"\"), Err(Err::Error((\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Tag))));"] # [doc = " ```"] pub fn separated_pair < I , O1 , O2 , E : ParseError < I > , F , G , H > (first : F , sep : G , second : H ,) -> impl Parser < I , Output = (O1 , O2) , Error = E > where F : Parser < I , Output = O1 , Error = E > , G : Parser < I , Error = E > , H : Parser < I , Output = O2 , Error = E > , { first . and (preceded (sep , second)) }
    };
}

separated_pair!();