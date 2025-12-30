// Generated macro for hex_digit0 (function)
macro_rules! Depcrate_character_completehex_digit0 {
() => {
// Module: crate::character::complete
// Provides: {"hex_digit0"}
// Dependencies: {}
# [doc = " Recognizes zero or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f"] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non hexadecimal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::hex_digit0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     hex_digit0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ\"), Ok((\"Z\", \"21c\")));"] # [doc = " assert_eq!(parser(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn hex_digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_hex_digit ()) }
};
}
