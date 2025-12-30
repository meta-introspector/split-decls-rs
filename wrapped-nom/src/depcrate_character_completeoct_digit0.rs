// Generated macro for oct_digit0 (function)
macro_rules! Depcrate_character_completeoct_digit0 {
() => {
// Module: crate::character::complete
// Provides: {"oct_digit0"}
// Dependencies: {}
# [doc = " Recognizes zero or more octal characters: 0-7"] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non octal"] # [doc = " digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::oct_digit0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     oct_digit0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ\"), Ok((\"cZ\", \"21\")));"] # [doc = " assert_eq!(parser(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn oct_digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_oct_digit ()) }
};
}
