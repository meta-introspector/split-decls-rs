// Generated macro for digit0 (function)
macro_rules! Depcrate_character_completedigit0 {
() => {
// Module: crate::character::complete
// Provides: {"digit0"}
// Dependencies: {}
# [doc = " Recognizes zero or more ASCII numerical characters: 0-9"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::digit0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     digit0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21c\"), Ok((\"c\", \"21\")));"] # [doc = " assert_eq!(parser(\"21\"), Ok((\"\", \"21\")));"] # [doc = " assert_eq!(parser(\"a21c\"), Ok((\"a21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_dec_digit ()) }
};
}
