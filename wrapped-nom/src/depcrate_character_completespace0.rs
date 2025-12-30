// Generated macro for space0 (function)
macro_rules! Depcrate_character_completespace0 {
() => {
// Module: crate::character::complete
// Provides: {"space0"}
// Dependencies: {}
# [doc = " Recognizes zero or more spaces and tabs."] # [doc = ""] # [doc = " *Complete version*: Will return the whole input if no terminating token is found (a non space"] # [doc = " character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::space0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     space0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\" \\t21c\"), Ok((\"21c\", \" \\t\")));"] # [doc = " assert_eq!(parser(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn space0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar + Clone , { input . split_at_position_complete (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t') }) }
};
}
