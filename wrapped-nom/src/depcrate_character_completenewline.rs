// Generated macro for newline (function)
macro_rules! Depcrate_character_completenewline {
() => {
// Module: crate::character::complete
// Provides: {"newline"}
// Dependencies: {}
# [doc = " Matches a newline character '\\n'."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::newline;"] # [doc = " fn parser(input: &str) -> IResult<&str, char> {"] # [doc = "     newline(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"\\nc\"), Ok((\"c\", '\\n')));"] # [doc = " assert_eq!(parser(\"\\r\\nc\"), Err(Err::Error(Error::new(\"\\r\\nc\", ErrorKind::Char))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Char))));"] # [doc = " ```"] pub fn newline < I , Error : ParseError < I > > (input : I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , { char ('\n') (input) }
};
}
