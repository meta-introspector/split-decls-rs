// Generated macro for pair (function)
macro_rules! Depcrate_sequencepair {
() => {
// Module: crate::sequence
// Provides: {"pair"}
// Dependencies: {}
# [doc = " Gets an object from the first parser,"] # [doc = " then gets another object from the second parser."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `first` The first parser to apply."] # [doc = " * `second` The second parser to apply."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use nom::sequence::pair;"] # [doc = " use nom::bytes::complete::tag;"] # [doc = " use nom::{error::ErrorKind, Err, Parser};"] # [doc = ""] # [doc = " let mut parser = pair(tag(\"abc\"), tag(\"efg\"));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"abcefg\"), Ok((\"\", (\"abc\", \"efg\"))));"] # [doc = " assert_eq!(parser.parse(\"abcefghij\"), Ok((\"hij\", (\"abc\", \"efg\"))));"] # [doc = " assert_eq!(parser.parse(\"\"), Err(Err::Error((\"\", ErrorKind::Tag))));"] # [doc = " assert_eq!(parser.parse(\"123\"), Err(Err::Error((\"123\", ErrorKind::Tag))));"] # [doc = " ```"] pub fn pair < I , O1 , O2 , E : ParseError < I > , F , G > (first : F , second : G ,) -> impl Parser < I , Output = (O1 , O2) , Error = E > where F : Parser < I , Output = O1 , Error = E > , G : Parser < I , Output = O2 , Error = E > , { first . and (second) }
};
}
