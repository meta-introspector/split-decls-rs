// Generated macro for multispace0 (function)
macro_rules! Depcrate_character_streamingmultispace0 {
() => {
// Module: crate::character::streaming
// Provides: {"multispace0"}
// Dependencies: {}
# [doc = " Recognizes zero or more spaces, tabs, carriage returns and line feeds."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::multispace0;"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\" \\t\\n\\r21c\"), Ok((\"21c\", \" \\t\\n\\r\")));"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(multispace0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn multispace0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t' || c == '\r' || c == '\n') }) }
};
}
