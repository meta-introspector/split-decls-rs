// Generated macro for bin_digit0 (function)
macro_rules! Depcrate_character_completebin_digit0 {
() => {
// Module: crate::character::complete
// Provides: {"bin_digit0"}
// Dependencies: {}
# [doc = " Recognizes zero or more binary characters: 0-1"] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non binary"] # [doc = " digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::bin_digit0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     bin_digit0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"013a\"), Ok((\"3a\", \"01\")));"] # [doc = " assert_eq!(parser(\"a013\"), Ok((\"a013\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn bin_digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | ! item . is_bin_digit ()) }
};
}
