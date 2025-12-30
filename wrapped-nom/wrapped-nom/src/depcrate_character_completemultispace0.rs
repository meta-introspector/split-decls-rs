// Generated macro for multispace0 (function)
macro_rules! Depcrate_character_completemultispace0 {
() => {
// Module: crate::character::complete
// Provides: {"multispace0"}
// Dependencies: {}
# [doc = " Recognizes zero or more spaces, tabs, carriage returns and line feeds."] # [doc = ""] # [doc = " *Complete version*: will return the whole input if no terminating token is found (a non space"] # [doc = " character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::complete::multispace0;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     multispace0(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\" \\t\\n\\r21c\"), Ok((\"21c\", \" \\t\\n\\r\")));"] # [doc = " assert_eq!(parser(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] pub fn multispace0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position_complete (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t' || c == '\r' || c == '\n') }) }
};
}
