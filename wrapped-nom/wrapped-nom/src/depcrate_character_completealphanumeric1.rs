// Generated macro for alphanumeric1 (function)
macro_rules! Depcrate_character_completealphanumeric1 {
() => {
// Module: crate::character::complete
// Provides: {"alphanumeric1"}
// Dependencies: {}
# [doc = " Recognizes one or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non alphanumerical character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::alphanumeric1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     alphanumeric1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ%1\"), Ok((\"%1\", \"21cZ\")));"] # [doc = " assert_eq!(parser(\"&H2\"), Err(Err::Error(Error::new(\"&H2\", ErrorKind::AlphaNumeric))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::AlphaNumeric))));"] # [doc = " ```"] pub fn alphanumeric1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | ! item . is_alphanum () , ErrorKind :: AlphaNumeric) }
};
}
