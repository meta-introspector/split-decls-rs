// Generated macro for tab (function)
macro_rules! Depcrate_character_completetab {
() => {
// Module: crate::character::complete
// Provides: {"tab"}
// Dependencies: {}
# [doc = " Matches a tab character '\\t'."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::tab;"] # [doc = " fn parser(input: &str) -> IResult<&str, char> {"] # [doc = "     tab(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"\\tc\"), Ok((\"c\", '\\t')));"] # [doc = " assert_eq!(parser(\"\\r\\nc\"), Err(Err::Error(Error::new(\"\\r\\nc\", ErrorKind::Char))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Char))));"] # [doc = " ```"] pub fn tab < I , Error : ParseError < I > > (input : I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , { char ('\t') (input) }
};
}
