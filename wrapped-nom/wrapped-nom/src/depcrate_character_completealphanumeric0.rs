// Generated macro for alphanumeric0 (function)
macro_rules! Depcrate_character_completealphanumeric0 {
() => {
// Module: crate::character::complete
// Provides: {"alphanumeric0"}
// Dependencies: {}
# [doc = " Recognizes zero or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z"] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non"] # [doc = " alphanumerical character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::alphanumeric0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     alphanumeric0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ%1\"), Ok((\"%1\", \"21cZ\")));"] # [doc = " assert_eq!(parser(\"&Z21c\"), Ok((\"&Z21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn alphanumeric0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_alphanum ()) }
};
}
