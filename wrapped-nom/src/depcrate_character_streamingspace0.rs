// Generated macro for space0 (function)
macro_rules! Depcrate_character_streamingspace0 {
() => {
// Module: crate::character::streaming
// Provides: {"space0"}
// Dependencies: {}
# [doc = " Recognizes zero or more spaces and tabs."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::space0;"] # [doc = " assert_eq!(space0::<_, (_, ErrorKind)>(\" \\t21c\"), Ok((\"21c\", \" \\t\")));"] # [doc = " assert_eq!(space0::<_, (_, ErrorKind)>(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(space0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn space0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t') }) }
};
}
