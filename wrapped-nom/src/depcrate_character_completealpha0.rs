// Generated macro for alpha0 (function)
macro_rules! Depcrate_character_completealpha0 {
() => {
// Module: crate::character::complete
// Provides: {"alpha0"}
// Dependencies: {}
# [doc = " Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z"] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non"] # [doc = " alphabetic character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::alpha0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     alpha0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"ab1c\"), Ok((\"1c\", \"ab\")));"] # [doc = " assert_eq!(parser(\"1c\"), Ok((\"1c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn alpha0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_alpha ()) }
};
}
